#[cfg(test)]
mod test {

    use rbatis_sql::rb_py;
    use rbson::{Bson, Document};

    #[rb_py("
    SELECT * FROM biz_activity
    if  name != null:
      AND delete_flag = #{del}
      AND version = 1
      if  age!=1:
        AND version = 1
      AND version = 1
    AND a = 0
      yes
    and id in (
    trim ',': for item in ids:
      #{item},
    )
    and id in (
    trim ',': for index,item in ids:
      #{item},
    )
    trim 'AND':
      AND delete_flag = #{del2}
    choose:
        when age==27:
          AND age = 27
        otherwise:
          AND age = 0
    WHERE id  = '2';")]
    pub fn py_select_by_condition(arg: &mut bson::Bson, _tag: char) {}

    #[test]
    fn test_rbatis_sql(){
        let mut arg =Document::new();
        arg.insert("name","ss");
        let (sql, args) = py_select_by_condition(&mut Bson::Document(arg), '$');
        println!("py->sql: {}", sql);
        println!("py->args: {}", serde_json::to_string(&args).unwrap());
    }
}