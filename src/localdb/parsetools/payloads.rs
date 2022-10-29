use super::{Payload, SelectPayload};

type Affected = usize;

pub trait ParsablePayload {
    fn maybe_next_select(&mut self) -> Option<SelectPayload>;
    fn try_next_select(&mut self) -> Result<SelectPayload, ParsePayloadError>;
    fn maybe_next_insert(&mut self) -> Option<Affected>;
    fn try_next_insert(&mut self) -> Result<Affected, ParsePayloadError>;
    fn maybe_next_update(&mut self) -> Option<Affected>;
    fn try_next_update(&mut self) -> Result<Affected, ParsePayloadError>;
    fn maybe_next_delete(&mut self) -> Option<Affected>;
    fn try_next_delete(&mut self) -> Result<Affected, ParsePayloadError>;
}

impl ParsablePayload for Vec<Payload> {
    fn maybe_next_select(&mut self) -> Option<SelectPayload> {
        let mut select: Option<SelectPayload> = None;
        for (idx, payload) in self.iter().enumerate() {
            match payload {
                Payload::Select { labels, rows } => {
                    select = Some(SelectPayload {
                        labels: labels.to_vec(),
                        rows: rows.to_vec(),
                    });
                    self.remove(idx);
                    break;
                }
                _ => (),
            }
        }
        select
    }

    fn try_next_select(&mut self) -> Result<SelectPayload, ParsePayloadError> {
        let mut found: bool = false;
        let mut select: SelectPayload = SelectPayload::default();
        for (idx, payload) in self.iter().enumerate() {
            match payload {
                Payload::Select { labels, rows } => {
                    select = SelectPayload {
                        labels: labels.to_vec(),
                        rows: rows.to_vec(),
                    };
                    found = true;
                    self.remove(idx);
                    break;
                }
                _ => (),
            }
        }
        if found {
            Ok(select)
        } else {
            Err(ParsePayloadError::SelectPayloadNotFoundError)
        }
    }

    fn maybe_next_insert(&mut self) -> Option<Affected> {
        let mut insert: Option<Affected> = None;
        for (idx, payload) in self.iter().enumerate() {
            match payload {
                Payload::Insert(num) => {
                    insert = Some(*num);
                    self.remove(idx);
                    break;
                }
                _ => (),
            }
        }
        insert
    }

    fn try_next_insert(&mut self) -> Result<Affected, ParsePayloadError> {
        let mut found: bool = false;
        let mut insert: Affected = usize::default();
        for (idx, payload) in self.iter().enumerate() {
            match payload {
                Payload::Insert(num) => {
                    insert = *num;
                    found = true;
                    self.remove(idx);
                    break;
                }
                _ => (),
            }
        }
        if found {
            Ok(insert)
        } else {
            Err(ParsePayloadError::InsertPayloadNotFoundError)
        }
    }

    fn maybe_next_update(&mut self) -> Option<Affected> {
        let mut update: Option<Affected> = None;
        for (idx, payload) in self.iter().enumerate() {
            match payload {
                Payload::Update(num) => {
                    update = Some(*num);
                    self.remove(idx);
                    break;
                }
                _ => (),
            }
        }
        update
    }

    fn try_next_update(&mut self) -> Result<Affected, ParsePayloadError> {
        let mut found: bool = false;
        let mut update: Affected = usize::default();
        for (idx, payload) in self.iter().enumerate() {
            match payload {
                Payload::Insert(num) => {
                    update = *num;
                    found = true;
                    self.remove(idx);
                    break;
                }
                _ => (),
            }
        }
        if found {
            Ok(update)
        } else {
            Err(ParsePayloadError::UpdatePayloadNotFoundError)
        }
    }

    fn maybe_next_delete(&mut self) -> Option<Affected> {
        let mut delete: Option<Affected> = None;
        for (idx, payload) in self.iter().enumerate() {
            match payload {
                Payload::Delete(num) => {
                    delete = Some(*num);
                    self.remove(idx);
                    break;
                }
                _ => (),
            }
        }
        delete
    }

    fn try_next_delete(&mut self) -> Result<Affected, ParsePayloadError> {
        let mut found: bool = false;
        let mut delete: Affected = usize::default();
        for (idx, payload) in self.iter().enumerate() {
            match payload {
                Payload::Insert(num) => {
                    delete = *num;
                    found = true;
                    self.remove(idx);
                    break;
                }
                _ => (),
            }
        }
        if found {
            Ok(delete)
        } else {
            Err(ParsePayloadError::DeletePayloadNotFoundError)
        }
    }
}

#[derive(Debug)]
pub enum ParsePayloadError {
    SelectPayloadNotFoundError,
    InsertPayloadNotFoundError,
    UpdatePayloadNotFoundError,
    DeletePayloadNotFoundError,
}

impl std::fmt::Display for ParsePayloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SelectPayloadNotFoundError => write!(f, "{:?}", "no select payload found"),
            Self::InsertPayloadNotFoundError => write!(f, "{:?}", "no insert payload found"),
            Self::UpdatePayloadNotFoundError => write!(f, "{:?}", "no update payload found"),
            Self::DeletePayloadNotFoundError => write!(f, "{:?}", "no delete payload found"),
        }
    }
}

impl std::error::Error for ParsePayloadError {}
