pub mod query;

#[macro_use]
extern crate lazy_static;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    
    use std::time::SystemTime;

    use crate::query::{
        operator::Op,
        select::Select,
        value::{Func, Value},
    
    };



    #[test]
    fn query() {

        let result = "        
            SELECT DISTINCT id, age, fullname FROM customer\nINNER JOIN merchant ON customer.id = merchant.customer_id\nLEFT JOIN product ON customer.id = product.customer_id\nWHERE age BETWEEN 10 AND 25\nAND fullname LIKE 'full%'\nAND fullname NOT IN ('danyal', 'danyalmh', 'danyalai')\nGROUP BY merchant_id\nHAVING COUNT(id) \nORDER BY fullname ASC, age DESC\nLIMIT 10\nOFFSET 5        
        ";

        let func_count_id = Func::Count("id").to_string();

        let start = SystemTime::now();
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
            .having(&func_count_id, Op::None)            
            .order_by("fullname")
            .order_by_desc("age")
            .limit(10)
            .offset(5);
        let end = SystemTime::now();

        assert_eq!(query.build().trim(), result.trim());
        println!("\n{}\n\n{:?}", query, end.duration_since(start))
    }


    #[test]
    fn cached_query() {
        
        let result = "        
            SELECT DISTINCT id, age, fullname FROM customer\nINNER JOIN merchant ON customer.id = merchant.customer_id\nLEFT JOIN product ON customer.id = product.customer_id\nWHERE age BETWEEN 10 AND 25\nAND fullname LIKE 'full%'\nAND fullname NOT IN ('danyal', 'danyalmh', 'danyalai')\nGROUP BY merchant_id\nHAVING COUNT(id) \nORDER BY fullname ASC, age DESC\nLIMIT 10\nOFFSET 5        
        ";

        let func_count_id = Func::Count("id").to_string();

        let query_ref = 
            Select::
                cols(vec!["id, age, fullname"])
                .distinct()
                .from("customer")
                .inner_join("merchant").on("customer.id", "customer_id")
                .left_join("product").on("customer.id", "customer_id")
                .where_by("age", Op::Between(Value::Param, Value::Param))
                .and("fullname", Op::Like(Value::param_str()))
                .and("fullname", Op::NotIn(vec![Value::Param, Value::Param, Value::Param].into()))
                .group_by(vec!["merchant_id"])
                .having(&func_count_id, Op::None)            
                .order_by("fullname")
                .order_by_desc("age")
                .limit(10)
                .offset(5)
                .prepare();



        let query = Select::stmt(&query_ref).unwrap();

        let start = SystemTime::now();
        
        let res = query.bind(vec![
            10.into(), 
            25.into(), 
            "full%".into(),
            "danyal".into(),
            "danyalmh".into(),
            "danyalai".into()
        ]).unwrap();
        
        let end = SystemTime::now();
        println!("{}\n\n===> {:?}", res, end.duration_since(start));


        assert_eq!(res.trim(), result.trim());

    }   


}
