///gen sql => INSERT INTO table_name (column1,column2,column3,...) VALUES (value1,value2,value3,...);
///
/// example:
/// pub struct BizActivity{}
///
/// impl_insert!(BizActivity,"biz_activity");
///
/// let table = BizActivity{}
/// table.insert()
///
#[macro_export]
macro_rules! impl_insert {
    ($table:ty{}) => {
        $crate::impl_insert!($table,$crate::utils::string_util::to_snake_name(stringify!($table)));
    };
    ($table:ty,$table_name:expr) => {
        impl $table{
            pub async fn insert(rb: &mut dyn $crate::executor::Executor,table: &$table)->Result<rbdc::db::ExecResult,rbdc::Error>{
                #[py_sql(
"insert into ${table_name} (
             trim ',':
               for k,v in table:
                  if k == 'id' && v== null:
                    #{continue}
                 ${k},
             ) VALUES (
             trim ',':
               for k,v in table:
                  if k == 'id' && v== null:
                    #{continue}
                 #{v},
             )")]
async fn do_insert(rb: &mut dyn $crate::executor::Executor,table: &$table,table_name:String) -> Result<rbdc::db::ExecResult,rbdc::Error> {impled!()}
             let table_name = $table_name.to_string();
             do_insert(rb.into(),table,table_name).await
            }
        }
    };
}


///gen sql => SELECT (column1,column2,column3,...) FROM table_name (column1,column2,column3,...)  *** WHERE ***
///
/// example:
/// pub struct BizActivity{}
///
/// impl_insert!(BizActivity,"biz_activity");
///
/// impl_select_all!(BizActivity,select_all_by_id,"select * from biz_activity where id = #{id}",id:String);
///
/// let table = BizActivity{}
/// BizActivity::select()
///
#[macro_export]
macro_rules! impl_select {
    ($table:ty{}) => {
        $crate::impl_select!($table,$crate::utils::string_util::to_snake_name(stringify!($table)));
    };
    ($table:ty,$table_name:expr) => {
        impl $table{
            pub async fn select_all(rb: &mut dyn  $crate::executor::Executor)->Result<Vec<$table>,rbdc::Error>{
                #[py_sql(
"select * from ${table_name}")]
async fn do_select_all(rb: &mut dyn $crate::executor::Executor,table_name:String) -> Result<Vec<$table>,rbdc::Error> {impled!()}
            let table_name = $table_name.to_string();
            do_select_all(rb,table_name).await
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)+) => $sql:expr}) => {
        impl $table{
            pub async fn $fn_name(rb: &mut dyn  $crate::executor::Executor,$($param_key:$param_type,)+)->Result<Vec<$table>,rbdc::Error>{
                #[py_sql($sql)]
async fn do_select_all(rb: &mut dyn $crate::executor::Executor,$($param_key:$param_type,)+) -> Result<Vec<$table>,rbdc::Error> {impled!()}
            do_select_all(rb,$($param_key ,)+).await
            }
        }
    };
    // select to an container
    // for example:
    // impl_select!(BizActivity{select_by_id(id:String) -> Option => "select * from biz_activity where id = #{id} limit 1"});
    // impl_select!(BizActivity{select_by_id(id:String) -> HashMap => "select * from biz_activity where id = #{id} limit 1"});
    // impl_select!(BizActivity{select_by_id(id:String) -> Vec => "select * from biz_activity where id = #{id} limit 1"});
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)+) -> $container:tt => $sql:expr}) => {
        impl $table{
            pub async fn $fn_name(rb: &mut dyn  $crate::executor::Executor,$($param_key:$param_type,)+)->Result<$container<$table>,rbdc::Error>{
                #[py_sql($sql)]
async fn do_select_all(rb: &mut dyn $crate::executor::Executor,$($param_key:$param_type,)+) -> Result<$container<$table>,rbdc::Error> {impled!()}
            do_select_all(rb,$($param_key ,)+).await
            }
        }
    };
}




/// gen sql = UPDATE table_name SET column1=value1,column2=value2,... WHERE some_column=some_value;
#[macro_export]
macro_rules! impl_update {
    ($table:ty{}) => {
        $crate::impl_update!($table,$crate::utils::string_util::to_snake_name(stringify!($table)));
    };
    ($table:ty,$table_name:expr) => {
        impl $table{
            pub async fn update_by_column(rb: &mut dyn $crate::executor::Executor,table:&$table,column:&str)->Result<rbdc::db::ExecResult,rbdc::Error>{
                #[py_sql(
"update ${table_name} set
             trim ',':
               for k,v in table:
                  if k == column || v== null:
                    #{continue}
                 ${k}=#{v},
             where  ${column} = #{column_value}   ")]
async fn do_update_by_column(rb: &mut dyn $crate::executor::Executor,table_name:String,table: &rbs::Value,column_value: &rbs::Value,column:&str) -> Result<rbdc::db::ExecResult,rbdc::Error> {impled!()}
            let table_name = $table_name.to_string();
            let table =  rbs::to_value!(table);
            let column_value = &table[column];
            do_update_by_column(rb,table_name,&table,column_value,column).await
            }
        }
    };
}


/// gen sql = DELETE FROM table_name WHERE some_column=some_value;
#[macro_export]
macro_rules! impl_delete {
    ($table:ty{}) => {
        $crate::impl_delete!($table,$crate::utils::string_util::to_snake_name(stringify!($table)));
    };
    ($table:ty,$table_name:expr) => {
        impl $table{
            pub async fn delete_by_column(rb: &mut dyn $crate::executor::Executor, column:&str,column_value: &rbs::Value)->Result<rbdc::db::ExecResult,rbdc::Error>{
            #[py_sql("delete from ${table_name} where  ${column} = #{column_value}")]
            async fn do_delete_by_column(rb: &mut dyn $crate::executor::Executor,table_name:String,column_value: &rbs::Value,column:&str) -> Result<rbdc::db::ExecResult,rbdc::Error> {impled!()}
            let table_name = $table_name.to_string();
            do_delete_by_column(rb,table_name,column_value,column).await
            }
        }
    };
}