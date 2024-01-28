// Code generated by Prisma Client Rust. DO NOT EDIT

pub static DATAMODEL_STR: &'static str =
    include_str!("/Users/lost/Projects/lunarstack/crates/prisma/schema.prisma");
static DATABASE_STR: &'static str = "postgres";
pub async fn new_client() -> Result<PrismaClient, ::prisma_client_rust::NewClientError> {
    PrismaClient::_builder().build().await
}
pub async fn new_client_with_url(
    url: &str,
) -> Result<PrismaClient, ::prisma_client_rust::NewClientError> {
    PrismaClient::_builder()
        .with_url(url.to_string())
        .build()
        .await
}
pub mod test_model {
    use super::_prisma::*;
    use super::*;
    pub const NAME: &str = "TestModel";
    pub mod id {
        use super::super::*;
        use super::_prisma::*;
        use super::{
            OrderByParam, SetParam, UncheckedSetParam, UniqueWhereParam, WhereParam, WithParam,
        };
        pub const NAME: &str = "id";
        pub struct Set(pub String);
        impl From<Set> for SetParam {
            fn from(Set(v): Set) -> Self {
                Self::SetId(v)
            }
        }
        impl From<Set> for UncheckedSetParam {
            fn from(Set(v): Set) -> Self {
                Self::Id(v)
            }
        }
        pub fn set<T: From<Set>>(value: String) -> T {
            Set(value).into()
        }
        pub fn order(direction: ::prisma_client_rust::Direction) -> OrderByParam {
            OrderByParam::Id(direction)
        }
        pub fn equals<T: From<UniqueWhereParam>>(value: String) -> T {
            UniqueWhereParam::IdEquals(value).into()
        }
        ::prisma_client_rust::scalar_where_param_fns!(_prisma::read_filters::StringFilter, Id, {
            fn in_vec(_: Vec<String>) -> InVec;
            fn not_in_vec(_: Vec<String>) -> NotInVec;
            fn lt(_: String) -> Lt;
            fn lte(_: String) -> Lte;
            fn gt(_: String) -> Gt;
            fn gte(_: String) -> Gte;
            fn contains(_: String) -> Contains;
            fn starts_with(_: String) -> StartsWith;
            fn ends_with(_: String) -> EndsWith;
            fn mode(_: super::super::QueryMode) -> Mode;
            fn not(_: String) -> Not;
        });
        pub struct Include;
        impl Into<super::IncludeParam> for Include {
            fn into(self) -> super::IncludeParam {
                super::IncludeParam::Id(self)
            }
        }
        impl Include {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
        pub struct Select;
        impl Into<super::SelectParam> for Select {
            fn into(self) -> super::SelectParam {
                super::SelectParam::Id(self)
            }
        }
        impl Select {
            pub fn to_selection(self) -> ::prisma_client_rust::Selection {
                ::prisma_client_rust::sel(NAME)
            }
        }
    }
    pub fn create(id: String, _params: Vec<SetParam>) -> (String, Vec<SetParam>) {
        (id, _params)
    }
    pub fn create_unchecked(id: String, _params: Vec<SetParam>) -> (String, Vec<SetParam>) {
        (id, _params)
    }
    #[macro_export]
    macro_rules ! _select_test_model { ($ (($ ($ func_arg : ident : $ func_arg_ty : ty) , +) =>) ? $ module_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { # [allow (warnings)] pub mod $ module_name { crate :: prisma :: test_model :: select ! (@ definitions ; $ module_name ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; use super :: * ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: SelectType for Selection { type Data = Data ; type ModelData = crate :: prisma :: test_model :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } pub fn select ($ ($ ($ func_arg : $ func_arg_ty) , +) ?) -> Selection { Selection ([crate :: prisma :: test_model :: select ! (@ selections_to_params ; : select { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () ,] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } } ; ({ $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { { crate :: prisma :: test_model :: select ! (@ definitions ; ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: SelectType for Selection { type Data = Data ; type ModelData = crate :: prisma :: test_model :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } Selection ([crate :: prisma :: test_model :: select ! (@ selections_to_params ; : select { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () ,] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } ; (@ definitions ; $ ($ module_name : ident) ? ; $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) +) => { # [allow (warnings)] enum Fields { id } # [allow (warnings)] impl Fields { fn selections () { $ (let _ = Fields :: $ field ;) + } } # [allow (warnings)] # [derive (std :: fmt :: Debug , Clone)] pub struct Data { $ (pub $ field : crate :: prisma :: test_model :: select ! (@ field_type ; $ field $ (: $ selection_mode { $ ($ selections) + }) ?) ,) + } impl :: serde :: Serialize for Data { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: Serializer , { use :: serde :: ser :: SerializeStruct ; let mut state = serializer . serialize_struct ("Data" , [$ (stringify ! ($ field) ,) +] . len ()) ? ; $ (state . serialize_field (crate :: prisma :: test_model :: $ field :: NAME , & self . $ field) ? ;) * state . end () } } impl < 'de > :: serde :: Deserialize < 'de > for Data { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : :: serde :: Deserializer < 'de > , { # [allow (warnings)] enum Field { $ ($ field) , + , } impl < 'de > :: serde :: Deserialize < 'de > for Field { fn deserialize < D > (deserializer : D) -> Result < Field , D :: Error > where D : :: serde :: Deserializer < 'de > , { struct FieldVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for FieldVisitor { type Value = Field ; fn expecting (& self , formatter : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { formatter . write_str (& [$ (crate :: prisma :: test_model :: $ field :: NAME) , + ,] . into_iter () . collect :: < Vec < _ >> () . join (", ")) } fn visit_str < E > (self , value : & str) -> Result < Field , E > where E : :: serde :: de :: Error , { match value { $ (crate :: prisma :: test_model :: $ field :: NAME => Ok (Field :: $ field)) , * , _ => Err (:: serde :: de :: Error :: unknown_field (value , FIELDS)) , } } } deserializer . deserialize_identifier (FieldVisitor) } } struct DataVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for DataVisitor { type Value = Data ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("struct Data") } fn visit_map < V > (self , mut map : V) -> Result < Data , V :: Error > where V : :: serde :: de :: MapAccess < 'de > , { $ (let mut $ field = None ;) * while let Some (key) = map . next_key () ? { match key { $ (Field :: $ field => { if $ field . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: test_model :: $ field :: NAME)) ; } $ field = Some (map . next_value () ?) ; }) * } } $ (let $ field = $ field . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: test_model :: $ field :: NAME)) ? ;) * Ok (Data { $ ($ field) , * }) } } const FIELDS : & 'static [& 'static str] = & ["id"] ; deserializer . deserialize_struct ("Data" , FIELDS , DataVisitor) } } $ ($ (pub mod $ field { crate :: prisma :: test_model :: $ selection_mode ! (@ field_module ; $ field : $ selection_mode { $ ($ selections) + }) ; }) ?) + } ; (@ field_type ; id) => { String } ; (@ field_type ; $ field : ident $ ($ tokens : tt) *) => { compile_error ! (stringify ! (Cannot include nonexistent relation $ field on model "TestModel" , available relations are "id")) } ; (@ field_module ; $ ($ tokens : tt) *) => { } ; (@ selection_field_to_selection_param ; id) => { Into :: < crate :: prisma :: test_model :: SelectParam > :: into (crate :: prisma :: test_model :: id :: Select) } ; (@ selection_field_to_selection_param ; $ ($ tokens : tt) *) => { compile_error ! (stringify ! ($ ($ tokens) *)) } ; (@ selections_to_params ; : $ macro_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { [$ (crate :: prisma :: test_model :: $ macro_name ! (@ selection_field_to_selection_param ; $ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) ,) +] } ; (@ filters_to_args ;) => { vec ! [] } ; (@ filters_to_args ; $ ($ t : tt) *) => { $ ($ t) * } ; (@ field_serde_name ; id) => { "id" } ; }
    pub use _select_test_model as select;
    pub enum SelectParam {
        Id(id::Select),
    }
    impl SelectParam {
        pub fn to_selection(self) -> ::prisma_client_rust::Selection {
            match self {
                Self::Id(data) => data.to_selection(),
            }
        }
    }
    #[macro_export]
    macro_rules ! _include_test_model { ($ (($ ($ func_arg : ident : $ func_arg_ty : ty) , +) =>) ? $ module_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { # [allow (warnings)] pub mod $ module_name { crate :: prisma :: test_model :: include ! (@ definitions ; $ module_name ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; use super :: * ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: IncludeType for Selection { type Data = Data ; type ModelData = crate :: prisma :: test_model :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } pub fn include ($ ($ ($ func_arg : $ func_arg_ty) , +) ?) -> Selection { Selection ([crate :: prisma :: test_model :: include ! (@ selections_to_params ; : include { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () , < crate :: prisma :: test_model :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections ()] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } } ; ({ $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { { crate :: prisma :: test_model :: include ! (@ definitions ; ; $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) +) ; pub struct Selection (Vec < :: prisma_client_rust :: Selection >) ; impl :: prisma_client_rust :: IncludeType for Selection { type Data = Data ; type ModelData = crate :: prisma :: test_model :: Data ; fn to_selections (self) -> Vec < :: prisma_client_rust :: Selection > { self . 0 } } Selection ([crate :: prisma :: test_model :: include ! (@ selections_to_params ; : include { $ ($ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) + }) . into_iter () . map (| p | p . to_selection ()) . collect :: < Vec < _ >> () , < crate :: prisma :: test_model :: Types as :: prisma_client_rust :: ModelTypes > :: scalar_selections ()] . into_iter () . flatten () . collect :: < Vec < _ >> ()) } } ; (@ definitions ; $ ($ module_name : ident) ? ; $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) +) => { # [allow (warnings)] enum Fields { } # [allow (warnings)] impl Fields { fn selections () { $ (let _ = Fields :: $ field ;) + } } # [allow (warnings)] # [derive (std :: fmt :: Debug , Clone)] pub struct Data { pub id : String , $ (pub $ field : crate :: prisma :: test_model :: include ! (@ field_type ; $ field $ (: $ selection_mode { $ ($ selections) + }) ?) ,) + } impl :: serde :: Serialize for Data { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : :: serde :: Serializer , { use :: serde :: ser :: SerializeStruct ; let mut state = serializer . serialize_struct ("Data" , [$ (stringify ! ($ field) ,) + stringify ! (id)] . len ()) ? ; $ (state . serialize_field (crate :: prisma :: test_model :: $ field :: NAME , & self . $ field) ? ;) * state . serialize_field (crate :: prisma :: test_model :: id :: NAME , & self . id) ? ; state . end () } } impl < 'de > :: serde :: Deserialize < 'de > for Data { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : :: serde :: Deserializer < 'de > , { # [allow (warnings)] enum Field { $ ($ field) , + , id } impl < 'de > :: serde :: Deserialize < 'de > for Field { fn deserialize < D > (deserializer : D) -> Result < Field , D :: Error > where D : :: serde :: Deserializer < 'de > , { struct FieldVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for FieldVisitor { type Value = Field ; fn expecting (& self , formatter : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { formatter . write_str (& [$ (crate :: prisma :: test_model :: $ field :: NAME) , + , crate :: prisma :: test_model :: id :: NAME] . into_iter () . collect :: < Vec < _ >> () . join (", ")) } fn visit_str < E > (self , value : & str) -> Result < Field , E > where E : :: serde :: de :: Error , { match value { $ (crate :: prisma :: test_model :: $ field :: NAME => Ok (Field :: $ field)) , * , crate :: prisma :: test_model :: id :: NAME => Ok (Field :: id) , _ => Err (:: serde :: de :: Error :: unknown_field (value , FIELDS)) , } } } deserializer . deserialize_identifier (FieldVisitor) } } struct DataVisitor ; impl < 'de > :: serde :: de :: Visitor < 'de > for DataVisitor { type Value = Data ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("struct Data") } fn visit_map < V > (self , mut map : V) -> Result < Data , V :: Error > where V : :: serde :: de :: MapAccess < 'de > , { $ (let mut $ field = None ;) * let mut id = None ; while let Some (key) = map . next_key () ? { match key { Field :: id => { if id . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: test_model :: id :: NAME)) ; } id = Some (map . next_value () ?) ; } $ (Field :: $ field => { if $ field . is_some () { return Err (:: serde :: de :: Error :: duplicate_field (crate :: prisma :: test_model :: $ field :: NAME)) ; } $ field = Some (map . next_value () ?) ; }) * } } $ (let $ field = $ field . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: test_model :: $ field :: NAME)) ? ;) * let id = id . ok_or_else (|| serde :: de :: Error :: missing_field (crate :: prisma :: test_model :: id :: NAME)) ? ; Ok (Data { id , $ ($ field) , * }) } } const FIELDS : & 'static [& 'static str] = & ["id"] ; deserializer . deserialize_struct ("Data" , FIELDS , DataVisitor) } } $ ($ (pub mod $ field { crate :: prisma :: test_model :: $ selection_mode ! (@ field_module ; $ field : $ selection_mode { $ ($ selections) + }) ; }) ?) + } ; (@ field_type ; $ field : ident $ ($ tokens : tt) *) => { compile_error ! (stringify ! (Cannot include nonexistent relation $ field on model "TestModel" , available relations are "")) } ; (@ field_module ; $ ($ tokens : tt) *) => { } ; (@ selection_field_to_selection_param ; $ ($ tokens : tt) *) => { compile_error ! (stringify ! ($ ($ tokens) *)) } ; (@ selections_to_params ; : $ macro_name : ident { $ ($ field : ident $ (($ ($ filters : tt) +) $ (. $ arg : ident ($ ($ arg_params : tt) *)) *) ? $ (: $ selection_mode : ident { $ ($ selections : tt) + }) ?) + }) => { [$ (crate :: prisma :: test_model :: $ macro_name ! (@ selection_field_to_selection_param ; $ field $ (($ ($ filters) +) $ (. $ arg ($ ($ arg_params) *)) *) ? $ (: $ selection_mode { $ ($ selections) + }) ?) ,) +] } ; (@ filters_to_args ;) => { vec ! [] } ; (@ filters_to_args ; $ ($ t : tt) *) => { $ ($ t) * } ; (@ field_serde_name ; id) => { "id" } ; }
    pub use _include_test_model as include;
    pub enum IncludeParam {
        Id(id::Include),
    }
    impl IncludeParam {
        pub fn to_selection(self) -> ::prisma_client_rust::Selection {
            match self {
                Self::Id(data) => data.to_selection(),
            }
        }
    }
    #[macro_export]
    macro_rules ! _partial_unchecked_test_model { ($ struct_name : ident { $ ($ scalar_field : ident) + }) => { :: prisma_client_rust :: macros :: partial_unchecked ! { crate :: prisma :: test_model struct $ struct_name { # [serde (rename = "id")] pub id : String } [$ ($ scalar_field) , +] } } ; }
    pub use _partial_unchecked_test_model as partial_unchecked;
    #[derive(Debug, Clone, :: serde :: Serialize, :: serde :: Deserialize)]
    pub struct Data {
        #[serde(rename = "id")]
        pub id: String,
    }
    impl Data {}
    #[derive(Clone)]
    pub enum WithParam {}
    impl Into<::prisma_client_rust::Selection> for WithParam {
        fn into(self) -> ::prisma_client_rust::Selection {
            match self {}
        }
    }
    #[derive(Clone)]
    pub enum SetParam {
        SetId(String),
    }
    impl From<SetParam> for (String, ::prisma_client_rust::PrismaValue) {
        fn from(param: SetParam) -> Self {
            match param {
                SetParam::SetId(value) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(value),
                ),
            }
        }
    }
    #[derive(Clone)]
    pub enum UncheckedSetParam {
        Id(String),
    }
    impl From<UncheckedSetParam> for SetParam {
        fn from(param: UncheckedSetParam) -> Self {
            match param {
                UncheckedSetParam::Id(value) => Self::SetId(value),
            }
        }
    }
    #[derive(Clone)]
    pub enum OrderByParam {
        Id(::prisma_client_rust::Direction),
    }
    impl Into<(String, ::prisma_client_rust::PrismaValue)> for OrderByParam {
        fn into(self) -> (String, ::prisma_client_rust::PrismaValue) {
            match self {
                Self::Id(direction) => (
                    id::NAME.to_string(),
                    ::prisma_client_rust::PrismaValue::String(direction.to_string()),
                ),
            }
        }
    }
    #[derive(Clone)]
    pub enum WhereParam {
        Not(Vec<WhereParam>),
        Or(Vec<WhereParam>),
        And(Vec<WhereParam>),
        Id(_prisma::read_filters::StringFilter),
    }
    impl ::prisma_client_rust::WhereInput for WhereParam {
        fn serialize(self) -> ::prisma_client_rust::SerializedWhereInput {
            let (name, value) = match self {
                Self::Not(value) => (
                    "NOT",
                    ::prisma_client_rust::SerializedWhereValue::Object(
                        ::prisma_client_rust::merge_fields(
                            value
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(Into::into)
                                .collect(),
                        ),
                    ),
                ),
                Self::Or(value) => (
                    "OR",
                    ::prisma_client_rust::SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(::prisma_client_rust::WhereInput::serialize)
                            .map(Into::into)
                            .map(|v| vec![v])
                            .map(::prisma_client_rust::PrismaValue::Object)
                            .collect(),
                    ),
                ),
                Self::And(value) => (
                    "AND",
                    ::prisma_client_rust::SerializedWhereValue::Object(
                        ::prisma_client_rust::merge_fields(
                            value
                                .into_iter()
                                .map(::prisma_client_rust::WhereInput::serialize)
                                .map(Into::into)
                                .collect(),
                        ),
                    ),
                ),
                Self::Id(value) => (id::NAME, value.into()),
            };
            ::prisma_client_rust::SerializedWhereInput::new(name, value.into())
        }
    }
    #[derive(Clone)]
    pub enum UniqueWhereParam {
        IdEquals(String),
    }
    impl From<UniqueWhereParam> for WhereParam {
        fn from(value: UniqueWhereParam) -> Self {
            match value {
                UniqueWhereParam::IdEquals(value) => {
                    Self::Id(_prisma::read_filters::StringFilter::Equals(value))
                }
            }
        }
    }
    impl From<::prisma_client_rust::Operator<Self>> for WhereParam {
        fn from(op: ::prisma_client_rust::Operator<Self>) -> Self {
            match op {
                ::prisma_client_rust::Operator::Not(value) => Self::Not(value),
                ::prisma_client_rust::Operator::And(value) => Self::And(value),
                ::prisma_client_rust::Operator::Or(value) => Self::Or(value),
            }
        }
    }
    #[derive(Clone)]
    pub struct Types;
    impl ::prisma_client_rust::ModelTypes for Types {
        type Data = Data;
        type Where = WhereParam;
        type UncheckedSet = UncheckedSetParam;
        type Set = SetParam;
        type With = WithParam;
        type OrderBy = OrderByParam;
        type Cursor = UniqueWhereParam;
        const MODEL: &'static str = NAME;
        fn scalar_selections() -> Vec<::prisma_client_rust::Selection> {
            vec![::prisma_client_rust::sel(id::NAME)]
        }
    }
    pub type UniqueArgs = ::prisma_client_rust::UniqueArgs<Types>;
    pub type ManyArgs = ::prisma_client_rust::ManyArgs<Types>;
    pub type Count<'a> = ::prisma_client_rust::Count<'a, Types>;
    pub type Create<'a> = ::prisma_client_rust::Create<'a, Types>;
    pub type CreateMany<'a> = ::prisma_client_rust::CreateMany<'a, Types>;
    pub type FindUnique<'a> = ::prisma_client_rust::FindUnique<'a, Types>;
    pub type FindMany<'a> = ::prisma_client_rust::FindMany<'a, Types>;
    pub type FindFirst<'a> = ::prisma_client_rust::FindFirst<'a, Types>;
    pub type Update<'a> = ::prisma_client_rust::Update<'a, Types>;
    pub type UpdateMany<'a> = ::prisma_client_rust::UpdateMany<'a, Types>;
    pub type Upsert<'a> = ::prisma_client_rust::Upsert<'a, Types>;
    pub type Delete<'a> = ::prisma_client_rust::Delete<'a, Types>;
    pub type DeleteMany<'a> = ::prisma_client_rust::DeleteMany<'a, Types>;
    #[derive(Clone)]
    pub struct Actions<'a> {
        pub client: &'a ::prisma_client_rust::PrismaClientInternals,
    }
    impl<'a> Actions<'a> {
        pub fn find_unique(self, _where: UniqueWhereParam) -> FindUnique<'a> {
            FindUnique::new(self.client, _where.into())
        }
        pub fn find_first(self, _where: Vec<WhereParam>) -> FindFirst<'a> {
            FindFirst::new(self.client, _where)
        }
        pub fn find_many(self, _where: Vec<WhereParam>) -> FindMany<'a> {
            FindMany::new(self.client, _where)
        }
        pub fn create(self, id: String, mut _params: Vec<SetParam>) -> Create<'a> {
            _params.extend([id::set(id)]);
            Create::new(self.client, _params)
        }
        pub fn create_unchecked(
            self,
            id: String,
            mut _params: Vec<UncheckedSetParam>,
        ) -> Create<'a> {
            _params.extend([id::set(id)]);
            Create::new(self.client, _params.into_iter().map(Into::into).collect())
        }
        pub fn create_many(self, data: Vec<(String, Vec<SetParam>)>) -> CreateMany<'a> {
            let data = data
                .into_iter()
                .map(|(id, mut _params)| {
                    _params.extend([id::set(id)]);
                    _params
                })
                .collect();
            CreateMany::new(self.client, data)
        }
        pub fn update(self, _where: UniqueWhereParam, _params: Vec<SetParam>) -> Update<'a> {
            Update::new(self.client, _where.into(), _params, vec![])
        }
        pub fn update_unchecked(
            self,
            _where: UniqueWhereParam,
            _params: Vec<UncheckedSetParam>,
        ) -> Update<'a> {
            Update::new(
                self.client,
                _where.into(),
                _params.into_iter().map(Into::into).collect(),
                vec![],
            )
        }
        pub fn update_many(
            self,
            _where: Vec<WhereParam>,
            _params: Vec<SetParam>,
        ) -> UpdateMany<'a> {
            UpdateMany::new(self.client, _where, _params)
        }
        pub fn upsert(
            self,
            _where: UniqueWhereParam,
            (id, mut _params): (String, Vec<SetParam>),
            _update: Vec<SetParam>,
        ) -> Upsert<'a> {
            _params.extend([id::set(id)]);
            Upsert::new(self.client, _where.into(), _params, _update)
        }
        pub fn delete(self, _where: UniqueWhereParam) -> Delete<'a> {
            Delete::new(self.client, _where.into(), vec![])
        }
        pub fn delete_many(self, _where: Vec<WhereParam>) -> DeleteMany<'a> {
            DeleteMany::new(self.client, _where)
        }
        pub fn count(self, _where: Vec<WhereParam>) -> Count<'a> {
            Count::new(self.client, _where)
        }
    }
}
pub mod _prisma {
    pub struct PrismaClientBuilder {
        url: Option<String>,
        action_notifier: ::prisma_client_rust::ActionNotifier,
    }
    impl PrismaClientBuilder {
        fn new() -> Self {
            Self {
                url: None,
                action_notifier: ::prisma_client_rust::ActionNotifier::new(),
            }
        }
        pub fn with_url(mut self, url: String) -> Self {
            self.url = Some(url);
            self
        }
        pub async fn build(self) -> Result<PrismaClient, ::prisma_client_rust::NewClientError> {
            let internals = ::prisma_client_rust::PrismaClientInternals::new(
                self.url,
                self.action_notifier,
                super::DATAMODEL_STR,
            )
            .await?;
            Ok(PrismaClient(internals))
        }
    }
    pub struct PrismaClient(::prisma_client_rust::PrismaClientInternals);
    impl ::std::fmt::Debug for PrismaClient {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.debug_struct("PrismaClient").finish()
        }
    }
    impl PrismaClient {
        pub fn _builder() -> PrismaClientBuilder {
            PrismaClientBuilder::new()
        }
        pub fn _query_raw<T: ::prisma_client_rust::Data>(
            &self,
            query: ::prisma_client_rust::Raw,
        ) -> ::prisma_client_rust::QueryRaw<T> {
            ::prisma_client_rust::QueryRaw::new(&self.0, query, super::DATABASE_STR)
        }
        pub fn _execute_raw(
            &self,
            query: ::prisma_client_rust::Raw,
        ) -> ::prisma_client_rust::ExecuteRaw {
            ::prisma_client_rust::ExecuteRaw::new(&self.0, query, super::DATABASE_STR)
        }
        pub async fn _batch<
            'batch,
            T: ::prisma_client_rust::BatchContainer<'batch, Marker>,
            Marker,
        >(
            &self,
            queries: T,
        ) -> ::prisma_client_rust::Result<
            <T as ::prisma_client_rust::BatchContainer<'batch, Marker>>::ReturnType,
        > {
            ::prisma_client_rust::batch(queries, &self.0).await
        }
        pub fn _transaction(&self) -> ::prisma_client_rust::TransactionBuilder<Self> {
            ::prisma_client_rust::TransactionBuilder::_new(self, &self.0)
        }
        pub fn test_model(&self) -> super::test_model::Actions {
            super::test_model::Actions { client: &self.0 }
        }
    }
    impl ::prisma_client_rust::PrismaClient for PrismaClient {
        fn internals(&self) -> &::prisma_client_rust::PrismaClientInternals {
            &self.0
        }
        fn internals_mut(&mut self) -> &mut ::prisma_client_rust::PrismaClientInternals {
            &mut self.0
        }
        fn with_tx_id(&self, tx_id: Option<::prisma_client_rust::query_core::TxId>) -> Self {
            Self(self.0.with_tx_id(tx_id))
        }
    }
    #[derive(Debug, Clone, Copy, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Eq)]
    pub enum QueryMode {
        #[serde(rename = "default")]
        Default,
        #[serde(rename = "insensitive")]
        Insensitive,
    }
    impl ToString for QueryMode {
        fn to_string(&self) -> String {
            match self {
                Self::Default => "default".to_string(),
                Self::Insensitive => "insensitive".to_string(),
            }
        }
    }
    #[derive(Debug, Clone, Copy, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Eq)]
    pub enum SortOrder {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
    impl ToString for SortOrder {
        fn to_string(&self) -> String {
            match self {
                Self::Asc => "asc".to_string(),
                Self::Desc => "desc".to_string(),
            }
        }
    }
    #[derive(Debug, Clone, Copy, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Eq)]
    pub enum TestModelScalarFieldEnum {
        #[serde(rename = "id")]
        Id,
    }
    impl ToString for TestModelScalarFieldEnum {
        fn to_string(&self) -> String {
            match self {
                Self::Id => "id".to_string(),
            }
        }
    }
    #[derive(Debug, Clone, Copy, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Eq)]
    pub enum TransactionIsolationLevel {
        #[serde(rename = "ReadUncommitted")]
        ReadUncommitted,
        #[serde(rename = "ReadCommitted")]
        ReadCommitted,
        #[serde(rename = "RepeatableRead")]
        RepeatableRead,
        #[serde(rename = "Serializable")]
        Serializable,
    }
    impl ToString for TransactionIsolationLevel {
        fn to_string(&self) -> String {
            match self {
                Self::ReadUncommitted => "ReadUncommitted".to_string(),
                Self::ReadCommitted => "ReadCommitted".to_string(),
                Self::RepeatableRead => "RepeatableRead".to_string(),
                Self::Serializable => "Serializable".to_string(),
            }
        }
    }
    impl ::prisma_client_rust::TransactionIsolationLevel for TransactionIsolationLevel {}
    pub mod read_filters {
        #[derive(Clone)]
        pub enum StringFilter {
            Equals(String),
            InVec(Vec<String>),
            NotInVec(Vec<String>),
            Lt(String),
            Lte(String),
            Gt(String),
            Gte(String),
            Contains(String),
            StartsWith(String),
            EndsWith(String),
            Mode(super::super::QueryMode),
            Not(String),
        }
        impl Into<::prisma_client_rust::SerializedWhereValue> for StringFilter {
            fn into(self) -> ::prisma_client_rust::SerializedWhereValue {
                match self {
                    Self::Equals(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "equals".to_string(),
                            ::prisma_client_rust::PrismaValue::String(value),
                        )])
                    }
                    Self::InVec(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "in".to_string(),
                            ::prisma_client_rust::PrismaValue::List(
                                value
                                    .into_iter()
                                    .map(|value| ::prisma_client_rust::PrismaValue::String(value))
                                    .collect(),
                            ),
                        )])
                    }
                    Self::NotInVec(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "notIn".to_string(),
                            ::prisma_client_rust::PrismaValue::List(
                                value
                                    .into_iter()
                                    .map(|value| ::prisma_client_rust::PrismaValue::String(value))
                                    .collect(),
                            ),
                        )])
                    }
                    Self::Lt(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "lt".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                    Self::Lte(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "lte".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                    Self::Gt(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "gt".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                    Self::Gte(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "gte".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                    Self::Contains(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "contains".to_string(),
                            ::prisma_client_rust::PrismaValue::String(value),
                        )])
                    }
                    Self::StartsWith(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "startsWith".to_string(),
                            ::prisma_client_rust::PrismaValue::String(value),
                        )])
                    }
                    Self::EndsWith(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "endsWith".to_string(),
                            ::prisma_client_rust::PrismaValue::String(value),
                        )])
                    }
                    Self::Mode(value) => {
                        ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                            "mode".to_string(),
                            ::prisma_client_rust::PrismaValue::Enum(value.to_string()),
                        )])
                    }
                    Self::Not(value) => ::prisma_client_rust::SerializedWhereValue::Object(vec![(
                        "not".to_string(),
                        ::prisma_client_rust::PrismaValue::String(value),
                    )]),
                }
            }
        }
    }
}
pub use _prisma::*;