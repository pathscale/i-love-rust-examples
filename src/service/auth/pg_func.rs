use model::types::*;

pub fn get_auth_pg_func() -> Vec<ProceduralFunction> {
    vec![
        ProceduralFunction::new(
            "fun_auth_signup",
            vec![
                Field::new("public_id", Type::BigInt),
                Field::new("username", Type::String),
                Field::new("password_hash", Type::Bytea),
                Field::new("password_salt", Type::Bytea),
                Field::new("age", Type::Int),
                Field::new("preferred_language", Type::String),
                Field::new("agreed_tos", Type::Boolean),
                Field::new("agreed_privacy", Type::Boolean),
                Field::new("ip_address", Type::Inet),
            ],
            vec![Field::new("user_id", Type::BigInt)],
            r#"
DECLARE
  id_ bigint;
BEGIN
  IF (a_agreed_tos = FALSE OR a_agreed_privacy = FALSE) THEN
    RAISE SQLSTATE 'R000X'; -- ConsentMissing
  ELSEIF ((SELECT pkey_id
           FROM tbl.user
           WHERE LOWER(username) = LOWER(a_username)) IS NOT NULL) THEN
    RAISE SQLSTATE 'R000Z'; -- UsernameAlreadyRegistered
  END IF;
  INSERT INTO tbl.user (public_id,
                       username,
                       password_hash,
                       password_salt,
                       age,
                       preferred_language,
                       agreed_tos,
                       agreed_privacy,
                       last_ip)
  VALUES (a_public_id,
          a_username,
          a_password_hash,
          a_password_salt,
          a_age,
          a_preferred_language,
          a_agreed_tos,
          a_agreed_privacy,
          a_ip_address)
  RETURNING pkey_id INTO STRICT id_;
  RETURN QUERY SELECT id_;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_auth_authenticate",
            vec![
                Field::new("username", Type::String),
                Field::new("password_hash", Type::Bytea),
                Field::new("service_code", Type::Int),
                Field::new("device_id", Type::String),
                Field::new("device_os", Type::String),
                Field::new("ip_address", Type::Inet),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("user_public_id", Type::BigInt),
            ],
            r#"
DECLARE
    is_blocked_     boolean;
    is_password_ok_ boolean;
    _user_id        bigint;
    _user_public_id bigint;
    _role           enum_role;
BEGIN
    ASSERT (a_ip_address NOTNULL AND a_device_id NOTNULL AND a_device_os NOTNULL AND
            a_username NOTNULL AND a_password_hash NOTNULL AND a_service_code NOTNULL);

    -- Looking up the user.
    SELECT pkey_id, u.public_id, is_blocked, (password_hash = a_password_hash), u.role
    INTO _user_id, _user_public_id, is_blocked_, is_password_ok_, _role
    FROM tbl.user u
    WHERE username = a_username;

    -- Log the login attempt.
    INSERT INTO tbl.login_attempt(fkey_user, username, password_hash, ip_address,
                                  is_password_ok)
    VALUES (_user_id, a_username, a_password_hash, a_ip_address, is_password_ok_);

    -- Checking the block status and password, and updating the login info if ok.
    IF (_user_id ISNULL) THEN
        RAISE SQLSTATE 'R0007'; -- UnknownUser
    END IF;
    IF (is_blocked_) THEN
        RAISE SQLSTATE 'R0008'; -- BlockedUser
    ELSIF (NOT is_password_ok_) THEN
        RAISE SQLSTATE 'R0009';
    ELSEIF (_role NOT IN ('admin', 'developer') AND
            a_service_code = (SELECT code FROM api.ADMIN_SERVICE())) OR
           (_role NOT IN ('user', 'admin', 'developer') AND
            a_service_code = (SELECT code FROM api.USER_SERVICE())) THEN
        RAISE SQLSTATE 'R000S'; -- InvalidRole
    END IF;
    UPDATE tbl.user -- ping
    SET last_ip      = a_ip_address,
        last_login   = EXTRACT(EPOCH FROM (NOW()))::bigint,
        logins_count = logins_count + 1
    WHERE pkey_id = _user_id;

    IF a_service_code = api.USER_SERVICE() THEN
        UPDATE tbl.user SET user_device_id = a_device_id WHERE pkey_id = _user_id;
    END IF;
    IF a_service_code = api.ADMIN_SERVICE() THEN
        UPDATE tbl.user SET admin_device_id = a_device_id WHERE pkey_id = _user_id;
    END IF;
    RETURN QUERY SELECT _user_id, _user_public_id;
END
        "#,
        ),
        ProceduralFunction::new(
            "fun_auth_get_password_salt",
            vec![Field::new("username", Type::String)],
            vec![Field::new("salt", Type::Bytea)],
            r#"
DECLARE
  user_id bigint;
BEGIN
  ASSERT (a_username NOTNULL);

  -- Looking up the user.
  SELECT pkey_id, u.password_salt
  INTO user_id, salt
  FROM tbl.user u
  WHERE username = a_username;

  IF (user_id ISNULL) THEN
    RAISE SQLSTATE 'R0007'; -- UnknownUser
  END IF;
  RETURN QUERY SELECT salt;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_set_token",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("user_token", Type::UUID),
                Field::new("admin_token", Type::UUID),
                Field::new("service_code", Type::Int),
            ],
            vec![],
            r#"
DECLARE
  rc_         integer;
  is_blocked_ boolean;
BEGIN
  ASSERT (a_user_id NOTNULL AND a_service_code NOTNULL AND a_user_token NOTNULL AND
          a_admin_token NOTNULL);
  -- Looking up the user.
  SELECT is_blocked INTO is_blocked_ FROM tbl.user WHERE pkey_id = a_user_id;
  IF (is_blocked_ ISNULL) THEN
    RAISE SQLSTATE 'R0007'; -- UnknownUser
  ELSIF (is_blocked_) THEN
    RAISE SQLSTATE 'R0008'; -- BlockedUser
  END IF;

  -- Setting up the token.
  IF a_service_code = (SELECT code FROM api.USER_SERVICE()) THEN
    UPDATE tbl.user
    SET user_token = a_user_token
    WHERE pkey_id = a_user_id;
  END IF;
  IF a_service_code = (SELECT code FROM api.ADMIN_SERVICE())  THEN
    UPDATE tbl.user
    SET admin_token = a_admin_token
    WHERE pkey_id = a_user_id;
  END IF;

  GET DIAGNOSTICS rc_ := ROW_COUNT;
  ASSERT (rc_ = 1);
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_authorize",
            vec![
                Field::new("username", Type::String),
                Field::new("token", Type::UUID),
                Field::new("service", Type::Enum("service".to_owned(), vec![])),
                Field::new("device_id", Type::String),
                Field::new("device_os", Type::String),
                Field::new("ip_address", Type::Inet),
            ],
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("role", Type::Enum("role".to_owned(), vec![])),
            ],
            r#"
DECLARE
    rc_          integer;
    is_token_ok_ boolean;
    user_id_     bigint;
    role_        enum_role;

BEGIN
    ASSERT (a_username NOTNULL AND a_token NOTNULL AND a_service NOTNULL AND
            a_device_id NOTNULL AND a_device_os NOTNULL);

    -- Looking up the user
    CASE a_service
        WHEN 'user'::enum_service
            THEN SELECT pkey_id, u.role, (user_token = a_token)
                 INTO user_id_, role_, is_token_ok_
                 FROM tbl.user AS u
                 WHERE username = a_username;
        WHEN 'admin'::enum_service
            THEN SELECT pkey_id, u.role, (admin_token = a_token)
                 INTO user_id_, role_, is_token_ok_
                 FROM tbl.user AS u
                 WHERE username = a_username;
        ELSE RAISE SQLSTATE 'R0001'; -- InvalidArgument
        END CASE;
    GET DIAGNOSTICS rc_ := ROW_COUNT;
    IF (rc_ <> 1) THEN
        RAISE SQLSTATE 'R0007'; -- UnknownUser
    END IF;

    -- Log the authorization attempt
    INSERT INTO tbl.authorization_attempt(fkey_user, ip_address, is_token_ok)
    VALUES (user_id_, a_ip_address, is_token_ok_ NOTNULL AND is_token_ok_);

    -- Validating the token
    IF NOT is_token_ok_ OR is_token_ok_ IS NULL THEN
        RAISE SQLSTATE 'R000A'; -- InvalidToken
    END IF;

    -- Updating the device info
    CASE a_service
        WHEN 'user'::enum_service
            THEN UPDATE tbl.user
                 SET user_device_id = a_device_id
                 WHERE pkey_id = user_id_
                   AND user_token = a_token;
        WHEN 'admin'::enum_service
            THEN UPDATE tbl.user
                 SET admin_device_id = a_device_id
                 WHERE pkey_id = user_id_
                   AND admin_token = a_token;
        END CASE;
    GET DIAGNOSTICS rc_ := ROW_COUNT;
    ASSERT (rc_ = 1);
    RETURN QUERY SELECT user_id_, role_;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_change_password",
            vec![
                Field::new("username", Type::String),
                Field::new("old_password_hash", Type::Bytea),
                Field::new("new_password_hash", Type::Bytea),
                Field::new("device_id", Type::String),
                Field::new("device_os", Type::String),
                Field::new("ip_address", Type::Inet),
            ],
            vec![],
            r#"
DECLARE
  is_blocked_     boolean;
  is_password_ok_ boolean;
  user_id_        bigint;
BEGIN
  ASSERT (a_ip_address NOTNULL AND a_device_id NOTNULL AND a_device_os NOTNULL AND
          a_username NOTNULL AND a_old_password_hash NOTNULL AND
          a_new_password_hash NOTNULL);
  -- Looking up the user.
  SELECT pkey_id, is_blocked, (password_hash = a_old_password_hash)
  INTO user_id_, is_blocked_, is_password_ok_
  FROM tbl.user u
  WHERE username = a_username;

  -- Log the login attempt.
  INSERT INTO tbl.login_attempt(fkey_user, username, password_hash, ip_address,
                                device_id, device_os, is_password_ok)
  VALUES (user_id_, a_username, a_old_password_hash, a_ip_address, a_device_id,
          a_device_os, is_password_ok_);

  -- Checking the block status and password, and updating the login info if ok.
  IF (user_id_ NOTNULL) THEN
    IF (is_blocked_) THEN
      RAISE SQLSTATE 'R0008'; -- BlockedUser
    ELSIF (NOT is_password_ok_) THEN
      RAISE SQLSTATE 'R0009'; -- InvalidPassword
    END IF;
    ASSERT (a_old_password_hash <> a_new_password_hash);

    UPDATE tbl.user
    SET password_hash = a_new_password_hash
    WHERE username = a_username;
  ELSE
      RAISE SQLSTATE 'R0007'; -- UnknownUser
  END IF;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_get_recovery_question_data",
            vec![],
            vec![
                Field::new("question_id", Type::Int),
                Field::new("content", Type::String),
                Field::new(
                    "category",
                    Type::Enum("recovery_question_category".to_owned(), vec![]),
                ),
            ],
            r#"
BEGIN
  RETURN QUERY SELECT q.pkey_id,
                      q.content,
                      q.category
               FROM tbl.recovery_question_data q;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_set_recovery_questions",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("question_ids", Type::Vec(Box::new(Type::Int))),
                Field::new("answers", Type::Vec(Box::new(Type::Int))),
            ],
            vec![],
            r#"
DECLARE
  rc_ integer;
BEGIN
  ASSERT (a_user_id NOTNULL AND a_question_ids NOTNULL AND a_answers NOTNULL);
  DELETE FROM tbl.recovery_question WHERE fkey_user = a_user_id;
  INSERT INTO tbl.recovery_question(fkey_user, fkey_question, answer)
  VALUES (a_user_id, UNNEST(a_question_ids), UNNEST(a_answers));
  GET DIAGNOSTICS rc_ := ROW_COUNT;
  ASSERT (rc_ > 0);
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_basic_authenticate",
            vec![
                Field::new("username", Type::String),
                Field::new("device_id", Type::String),
                Field::new("device_os", Type::String),
                Field::new("ip_address", Type::Inet),
            ],
            vec![Field::new("user_id", Type::Inet)],
            r#"
DECLARE
  is_blocked_ boolean;
  user_id_    bigint;
BEGIN
  ASSERT (a_username NOTNULL AND a_device_id NOTNULL AND a_device_os NOTNULL AND
          a_ip_address NOTNULL);
  SELECT pkey_id, is_blocked
  INTO user_id_, is_blocked_
  FROM tbl.user
  WHERE username = LOWER(a_username);
  INSERT INTO tbl.login_attempt(fkey_user, username, password_hash, ip_address,
                                device_id, device_os)
  VALUES (user_id_, a_username, '', a_ip_address, a_device_id, a_device_os);
  IF (user_id_ ISNULL) THEN
    RAISE SQLSTATE 'R0007'; -- UnknownUser
  ELSEIF (is_blocked_) THEN
    RAISE SQLSTATE 'R0008'; -- BlockedUser
  END IF;
  RETURN QUERY SELECT user_id_;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_get_recovery_questions",
            vec![Field::new("user_id", Type::BigInt)],
            vec![
                Field::new("question_id", Type::Int),
                Field::new("question", Type::String),
            ],
            r#"
BEGIN
  ASSERT (a_user_id NOTNULL);
  RETURN QUERY SELECT qd.pkey_id,
                      qd.content
               FROM tbl.recovery_question_data qd
                      JOIN tbl.recovery_question q ON qd.pkey_id = q.fkey_question
               WHERE q.fkey_user = a_user_id;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_submit_recovery_answers",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("question_ids", Type::Vec(Box::new(Type::Int))),
                Field::new("answers", Type::Vec(Box::new(Type::String))),
                Field::new("password_reset_token", Type::UUID),
                Field::new("token_valid", Type::Int),
            ],
            vec![],
            r#"
DECLARE
  correct_answers_ varchar[];
BEGIN
  IF (SELECT COUNT(pkey_id) FROM tbl.recovery_question WHERE fkey_user = a_user_id) !=
     CARDINALITY(a_question_ids) THEN
    RAISE SQLSTATE 'R0011'; -- MustSubmitAllRecoveryQuestions
  END IF;
  SELECT ARRAY_AGG(result.answer)
  INTO correct_answers_
  FROM (SELECT q.answer AS answer
        FROM tbl.recovery_question q
               JOIN UNNEST(a_question_ids) WITH ORDINALITY t(fkey_question, ord)
                    USING (fkey_question)
        WHERE fkey_user = a_user_id
        ORDER BY t.ord) result;
  IF a_answers != correct_answers_ THEN
    RAISE SQLSTATE 'R000T'; -- WrongRecoveryAnswers
  END IF;
  UPDATE tbl.user
  SET password_reset_token = a_password_reset_token,
      reset_token_valid    = a_token_valid
  WHERE pkey_id = a_user_id;
END
            "#,
        ),
        ProceduralFunction::new(
            "fun_auth_reset_password",
            vec![
                Field::new("user_id", Type::BigInt),
                Field::new("new_password_hash", Type::Bytea),
                Field::new("new_password_salt", Type::Bytea),
                Field::new("reset_token", Type::UUID),
            ],
            vec![],
            r#"
DECLARE
  rc_ integer;
BEGIN
  ASSERT (a_user_id NOTNULL AND a_new_password_hash NOTNULL AND a_reset_token NOTNULL);
  UPDATE tbl.user
  SET password_hash        = a_new_password_hash,
      password_salt        = a_new_password_salt,
      password_reset_token = NULL,
      reset_token_valid    = NULL
  WHERE pkey_id = a_user_id
    AND password_reset_token = a_reset_token
    AND reset_token_valid > EXTRACT(EPOCH FROM NOW())::bigint;
  GET DIAGNOSTICS rc_ := ROW_COUNT;
  IF (rc_ <> 1) THEN
    RAISE SQLSTATE 'R0012'; -- InvalidRecoveryToken
  END IF;
END
            "#,
        ),
    ]
}
