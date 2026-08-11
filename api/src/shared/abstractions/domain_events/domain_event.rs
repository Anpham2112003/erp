pub trait DomainEvent {
    fn raise_domain_events(&self);
    fn add_domain_event(&self, event: Box<dyn DomainEvent>);
}