use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use fluent_templates::{FluentLoader, ArcLoader};
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};
use unic_langid::langid;
use std::sync::{Arc, Mutex};
use tera::{Tera, Value};

pub struct ViewEngineInitializer;

#[async_trait]
impl Initializer for ViewEngineInitializer {
    fn name(&self) -> String {
        "view-engine".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![
            ("layout", include_str!("../../views/layout.html")),
            ("home/index", include_str!("../../views/home/index.html")),
        ])?;

        let tera_engine = Arc::new(Mutex::new(tera));
        // let arc = Arc::new(FluentLoader::new(&[langid!("en-US")]));
        let arc = Arc::new(FluentLoader::new(&[langid!("en-US")]));

        {
            let mut engine = tera_engine.lock().unwrap();
            engine.register_function("t", move |args: &std::collections::HashMap<String, Value>| {
                let key = args.get("key").unwrap().as_str().unwrap();
                Ok(Value::String(key.to_string()))
            // });
                // Ok(Value::String(arc.lookup(langid!("en-US"), key).unwrap_or_else(|| key.to_string())))
            });
        }

        Ok(router.layer(Extension(tera_engine)))
    }
}
