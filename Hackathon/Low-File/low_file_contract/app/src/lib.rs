#![no_std]

// necesary crates
use sails_rs::prelude::*;

// import our modules 
pub mod states;
pub mod services;

// Import service to be used for the program
use services::traffic_light_service::LowFileService;

// Traffic light program struct to build the program (there can only be one per contract)
pub struct TrafficLightProgram;

// Traffic light program, it host one or more services and it expose them to the 
// externar consumer.
// Only one program is allowed per application
#[program]
impl TrafficLightProgram {
    // Application constructor (it is an associated function)
    // It can be called once per application lifetime.
    pub fn new() -> Self {
        // Init the state
        //LowFileService::seed();

        Self
    }

    // Method working with "&self", having no other parameters are treated as exposed
    // service constructors, and are called each time when an incoming request message 
    // needs be dispatched to a selected service
    // It has "message routing", This will change the way a service will be called 
    // (if omitted, the method name will be used, in this case TrafficLightSvc).
    #[route("TrafficLight")]
    pub fn traffic_light_svc(&self) -> LowFileService {
        LowFileService::new("".to_string(), "".to_string(), 0, "".to_string(), "".to_string(), "".to_string(), Vec::new(),"".to_string(), Vec::new(),"".to_string(),"".to_string())
    }
}