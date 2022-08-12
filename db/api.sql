
CREATE OR REPLACE FUNCTION api.fun_auth_add(a_a int, a_b int)
RETURNS table (
sum int
)
LANGUAGE plpgsql
AS $$
    
BEGIN
    RETURN QUERY SELECT a_a + a_b AS sum;
END
        
$$;
        
