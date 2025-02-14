#[derive(Debug, Clone)]
pub enum Status {
    Alive(AliveStatus),
    Dead,
}

#[derive(Debug, Clone)]
pub enum AliveStatus {
    Healthy,
    Injured,
}
