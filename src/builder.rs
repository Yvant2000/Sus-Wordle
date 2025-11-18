use crate::context::Context;
use std::collections::HashMap;

#[derive(Default)]
pub struct SusBuilderBody<'a> {
    pub head: Option<&'a str>,
    pub eyes: Option<&'a str>,
    pub body: Option<&'a str>,
    pub legs: Option<&'a str>,
}

#[derive(Default)]
pub struct SusBuilder<'a> {
    pub empty: Option<&'a str>,
    pub body_ready: Option<Context>,
    pub body: HashMap<Context, SusBuilderBody<'a>>,
}

impl<'a> SusBuilderBody<'a> {
    pub fn is_full(&self) -> bool {
        self.head.is_some() && self.eyes.is_some() && self.body.is_some() && self.legs.is_some()
    }
}

impl<'a> SusBuilder<'a> {
    pub fn fill_head(&mut self, context: Context, value: &'a str) {
        let body_parts = self.body.entry(context.clone()).or_default();

        if body_parts.head.is_none() {
            body_parts.head = Some(value);
        }

        if body_parts.is_full() {
            self.body_ready = Some(context);
        }
    }

    pub fn fill_eyes(&mut self, context: Context, value: &'a str) {
        let body_parts = self.body.entry(context.clone()).or_default();

        if body_parts.eyes.is_none() {
            body_parts.eyes = Some(value);
        }

        if body_parts.is_full() {
            self.body_ready = Some(context);
        }
    }

    pub fn fill_body(&mut self, context: Context, value: &'a str) {
        let body_parts = self.body.entry(context.clone()).or_default();

        if body_parts.body.is_none() {
            body_parts.body = Some(value);
        }

        if body_parts.is_full() {
            self.body_ready = Some(context);
        }
    }

    pub fn fill_legs(&mut self, context: Context, value: &'a str) {
        let body_parts = self.body.entry(context.clone()).or_default();

        if body_parts.legs.is_none() {
            body_parts.legs = Some(value);
        }

        if body_parts.is_full() {
            self.body_ready = Some(context);
        }
    }

    pub fn is_full(&self) -> bool {
        self.empty.is_some() && self.body_ready.is_some()
    }

    /// Panics if is_full() == false
    pub fn convert(&self) -> [String; 6] {
        let body_context = self.body_ready.clone().unwrap();
        let body_parts = self.body.get(&body_context).unwrap();

        [
            self.empty.unwrap().to_owned(),
            body_parts.head.unwrap().to_owned(),
            body_parts.eyes.unwrap().to_owned(),
            body_parts.body.unwrap().to_owned(),
            body_parts.legs.unwrap().to_owned(),
            self.empty.unwrap().to_owned(),
        ]
    }
}
