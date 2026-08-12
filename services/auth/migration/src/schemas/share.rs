use sea_orm_migration::{prelude::*, schema::*};
#[derive(DeriveIden)]
pub enum Audit{
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    IsDeleted,
    DeletedAt,
    DeletedBy,
    IsActived,
    ActivedAt,
}

#[derive(DeriveIden)]
pub enum Search {
    TextSearch
}



//Trait 

pub trait AuditChange {
    fn audit_created(&mut self) -> &mut Self;
    fn audit_updated(&mut self) -> &mut Self;
    fn audit_deleted(&mut self) -> &mut Self;
    fn audit_actived(&mut self) -> &mut Self;
    fn audit_full(&mut self)-> &mut Self;
  
}


pub  trait AddSearch {
    fn add_text_search_colum(&mut self) -> &mut Self;
}


//End Trait




// Implement Trait

impl AuditChange for TableCreateStatement   {


    fn audit_created(&mut self) -> &mut Self {
        self
            .col(date_time(Audit::CreatedAt))
            .col(integer(Audit::CreatedBy))
    }

    fn audit_updated(&mut self) -> &mut Self {
        self
            .col(date_time(Audit::UpdatedAt))
            .col(integer(Audit::UpdatedBy))
    }

    fn audit_deleted(&mut self) -> &mut Self {
        self
            .col(boolean(Audit::IsDeleted))
            .col(date_time(Audit::DeletedAt))
            .col(integer(Audit::DeletedBy))
    }

    fn audit_actived(&mut self) -> &mut Self {
        self
            .col(boolean(Audit::IsActived))
            .col(date_time(Audit::ActivedAt))
    }

    fn audit_full(&mut self)-> &mut Self{
        self
            .audit_created()
            .audit_updated()
            .audit_deleted()
            .audit_actived()
    }

    
}

impl AddSearch for TableCreateStatement {
    fn add_text_search_colum(&mut self) -> &mut Self {
        self.col(text(Search::TextSearch))
    }
}

impl AddSearch for IndexCreateStatement  {
    fn add_text_search_colum(&mut self) -> &mut Self {
        self.col(Search::TextSearch)
    }
}





//End Implement Trait

