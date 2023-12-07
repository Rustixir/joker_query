

pub mod query;




#[cfg(test)]
mod tests {
    

    use crate::query::{
        operator as op,
        select::Select,
        func::Func, 
        insert::Insert, 
        update::Update, 
        set::Pair, 
        delete::Delete, array::Array,
    };


    #[test]
    fn select() {

        let result = "        
            SELECT DISTINCT id, age, fullname FROM customer\nINNER JOIN merchant ON customer.id = merchant.customer_id\nLEFT JOIN product ON customer.id = product.customer_id\nWHERE age BETWEEN 10 AND 25\nAND fullname LIKE 'full%'\nOR fullname NOT IN ('danyal', 'danyalmh', 'danyalai')\nGROUP BY merchant_id\nHAVING COUNT(id) = 2025\nORDER BY fullname ASC, age DESC\nLIMIT 10\nOFFSET 5        
        ";

        let query = 
        Select::
            cols(vec!["id", "age", "fullname"])
            .distinct()
            .from("customer")
            .inner_join("merchant").on("customer.id", "customer_id")
            .left_join("product").on("customer.id", "customer_id")
            .where_by("age", op::between(10, 25))
            .and("fullname", op::like("full%"))
            .or("fullname", op::not_in(vec!["danyal", "danyalmh", "danyalai"]))
            .group_by(vec!["merchant_id"])
            .having(&Func::count("id"), op::eq(2025))            
            .order_by("fullname")
            .order_by_desc("age")
            .limit(10)
            .offset(5)
            .build();

        assert_eq!(query.trim(), result.trim());
        println!("\n{}\n", query);

    }

    #[test]
    fn select_with_source() {
        let query = 
        Select::
            cols(vec!["id", "age", "fullname"])
            .distinct()
            .from_subquery(
                Select::
                    cols(vec!["id", "age", "fullname"])
                    .from("another_table")
                    .where_by("age", op::gt(22))
            )
            .where_by("age", op::between(10, 25))
            .order_by_desc("age")
            .limit(10)
            .offset(5)
            .build();
        
        println!("\n{}\n", query);

    }

    #[test]
    fn insert() {
        let query = 
            Insert::into("customer")
                .cols(vec!["id", "age", "fullname"])
                .value(Array::new().add("766dc50e").add(25).add("Danyal"))
                .build();

        println!("\n{}\n", query);
    }


    #[test]
    fn insert_values() {
        let query = 
            Insert::into("customer").cols(vec!["id", "age", "fullname"])
                .values(vec![
                    Array::new().add("766dc50e").add(25).add("Danyal"),
                    Array::new().add("766dc50e").add(25).add("Danyal"),
                ])
                .build();

        println!("\n{}\n", query);   
    }


    #[test]
    fn insert_value_select() {
        let query = 
            Insert::into("customer").cols(vec!["id", "age", "fullname"])
                .source(
                    Select::cols(vec!["id", "age", "fullname"]).from("customer_template")
                                .where_by("age", op::eq(10)).limit(10)
                )
                .build();

        println!("\n{}\n", query);  
            
    }


    #[test]
    pub fn update() {
        let query = 
         Update::table("customer")
            .set(vec![Pair::from("age", 100), Pair::from("fullname", "DanyalMh")])
            .where_by("id", op::eq(100))
                .and("fullname", op::not("DanyalMh"))
            .build();
            
        println!("\n{}\n", query)
    }

    #[test]
    pub fn delete() {
        let query = 
            Delete::from("customer")
                .where_by("fullname", op::neq("DanyalMh"))
                    .or("age", op::not_between(23, 25))
                .build();

        println!("\n{}\n", query)
    }


    #[test]
    pub fn delete_with_sub_query() {
        let query = 
            Delete::from("customer")
                .where_by("age", op::lt(
                    Select::cols(vec!["MIN(age)"]).from("athele")
                ))
                .build();

        println!("\n{}\n", query)
    }

}
