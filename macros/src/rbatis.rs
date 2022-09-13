
#[macro_export]
macro_rules! impl_select {
    ($table:ty{}) => {
        rbatis::impl_select!($table{},rbatis::utils::string_util::to_snake_name(stringify!($table)));
    };
    ($table:ty{},$table_name:expr) => {
        impl $table{
            pub async fn select_all(rb: &mut dyn  rbatis::executor::Executor)->Result<Vec<$table>,rbatis::rbdc::Error>{
                #[rbatis::py_sql("select * from ${table_name}")]
                async fn select_all(rb: &mut dyn rbatis::executor::Executor,table_name:String) -> Result<Vec<$table>,rbatis::rbdc::Error> {impled!()}
                let table_name = $table_name.to_string();
                select_all(rb,table_name).await
            }

            pub async fn select_by_column<V:serde::Serialize>(rb: &mut dyn  rbatis::executor::Executor, column: &str,column_value:V)->Result<Vec<$table>,rbatis::rbdc::Error>{
                #[rbatis::py_sql("select * from ${table_name} where ${column} = #{column_value}")]
                async fn select_by_column(rb: &mut dyn rbatis::executor::Executor,table_name:String, column:&str, column_value: &rbs::Value) -> Result<Vec<$table>,rbatis::rbdc::Error> {impled!()}
                let table_name = $table_name.to_string();
                let column_value = rbs::to_value!(column_value);
                select_by_column(rb,table_name,column,&column_value).await
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) => $sql:expr}) => {
        impl $table{
            pub async fn $fn_name(rb: &mut dyn  rbatis::executor::Executor,$($param_key:$param_type,)*)->Result<Vec<$table>,rbatis::rbdc::Error>{
                   #[rbatis::py_sql("`select ${table_column} from ${table_name} `",$sql)]
                   async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<Vec<$table>,rbatis::rbdc::Error> {impled!()}
                   let mut table_column = "*".to_string();
                   let mut table_name = rbatis::utils::string_util::to_snake_name(stringify!($table));
                   $fn_name(rb,&table_column,&table_name,$($param_key ,)*).await
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) =>$table_column:expr,$sql_where:expr}) => {
        impl $table{
            pub async fn $fn_name(rb: &mut dyn  rbatis::executor::Executor,$($param_key:$param_type,)*)->Result<Vec<$table>,rbatis::rbdc::Error>{
                   #[rbatis::py_sql("`select ${table_column} from ${table_name} `",$sql_where)]
                   async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<Vec<$table>,rbatis::rbdc::Error> {impled!()}
                   let mut table_name = rbatis::utils::string_util::to_snake_name(stringify!($table));
                   $fn_name(rb,&table_column,&table_name,$($param_key ,)*).await
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $container:tt => $sql:expr}) => {
        impl $table{
            pub async fn $fn_name(rb: &mut dyn  rbatis::executor::Executor,$($param_key:$param_type,)*)->Result<$container<$table>,rbatis::rbdc::Error>{
                     #[rbatis::py_sql("`select ${table_column} from ${table_name} `",$sql)]
                     async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<$container<$table>,rbatis::rbdc::Error> {impled!()}
                     let mut table_column = "*".to_string();
                     let mut table_name = rbatis::utils::string_util::to_snake_name(stringify!($table));
                     $fn_name(rb,&table_column,&table_name,$($param_key ,)*).await
            }
        }
    };
    ($table:ty{$fn_name:ident($($param_key:ident:$param_type:ty$(,)?)*) -> $container:tt => $table_column:expr,$sql_where:expr}) => {
        impl $table{
            pub async fn $fn_name(rb: &mut dyn  rbatis::executor::Executor,$($param_key:$param_type,)*)->Result<$container<$table>,rbatis::rbdc::Error>{
                     #[rbatis::py_sql("`select ${table_column} from ${table_name} `",$sql_where)]
                     async fn $fn_name(rb: &mut dyn rbatis::executor::Executor,table_column:&str,table_name:&str,$($param_key:$param_type,)*) -> Result<$container<$table>,rbatis::rbdc::Error> {impled!()}
                     let mut table_name = rbatis::utils::string_util::to_snake_name(stringify!($table));
                     $fn_name(rb,&table_column,&table_name,$($param_key ,)*).await
            }
        }
    };
}