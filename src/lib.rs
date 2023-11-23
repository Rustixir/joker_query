

pub mod query;




#[cfg(test)]
mod tests {
    

    use crate::query::{
        operator::Op,
        select::Select,
        func::Func,
    };



    #[test]
    fn query() {

        let result = "        
            SELECT DISTINCT id, age, fullname FROM customer\nINNER JOIN merchant ON customer.id = merchant.customer_id\nLEFT JOIN product ON customer.id = product.customer_id\nWHERE age BETWEEN 10 AND 25\nAND fullname LIKE 'full%'\nAND fullname NOT IN ('danyal', 'danyalmh', 'danyalai')\nGROUP BY merchant_id\nHAVING COUNT(id) = 1000\nORDER BY fullname ASC, age DESC\nLIMIT 10\nOFFSET 5        
        ";

    
        let query = 
        Select::
            cols(vec!["id, age, fullname"])
            .distinct()
            .from("customer")
            .inner_join("merchant").on("customer.id", "customer_id")
            .left_join("product").on("customer.id", "customer_id")
            .where_by("age", Op::Between(10.into(), 25.into()))
            .and("fullname", Op::Like("full%"))
            .and("fullname", Op::NotIn(vec!["danyal", "danyalmh", "danyalai"].into()))
            .group_by(vec!["merchant_id"])
            .having(&Func::Count("id").to_string(), Op::Eq(1000.into()))            
            .order_by("fullname")
            .order_by_desc("age")
            .limit(10)
            .offset(5)
            .build();

        assert_eq!(query.trim(), result.trim());
    }


    #[test]
    fn query_new_syntax() {

        let result = "        
            SELECT DISTINCT id, age, fullname FROM customer\nINNER JOIN merchant ON customer.id = merchant.customer_id\nLEFT JOIN product ON customer.id = product.customer_id\nWHERE age BETWEEN 10 AND 25\nAND fullname LIKE 'full%'\nAND fullname NOT IN ('danyal', 'danyalmh', 'danyalai')\nGROUP BY merchant_id\nHAVING COUNT(id) = 1000\nORDER BY fullname ASC, age DESC\nLIMIT 10\nOFFSET 5        
        ";

        let query = 
        Select::
            cols(vec!["id, age, fullname"])
            .distinct()
            .from("customer")
            .inner_join("merchant").on("customer.id", "customer_id")
            .left_join("product").on("customer.id", "customer_id")
            .where_by("age", Op::between(10, 25))
            .and("fullname", Op::like("full%"))
            .and("fullname", Op::not_in(vec!["danyal", "danyalmh", "danyalai"]))
            .group_by(vec!["merchant_id"])
            .having(&Func::count("id"), Op::eq(2025))            
            .order_by("fullname")
            .order_by_desc("age")
            .limit(10)
            .offset(5)
            .build();
        


        assert_eq!(query.trim(), result.trim());
    }


}
