
#[macro_use]
extern crate wascc_codec as codec;

#[macro_use]
extern crate log;

use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher, CapabilityDescriptor, OperationDirection, OP_GET_CAPABILITY_DESCRIPTOR};
use codec::core::{OP_BIND_ACTOR, OP_REMOVE_ACTOR, CapabilityConfiguration};
use codec::{serialize, deserialize};
use codec::messaging::BrokerMessage;
use codec::messaging::OP_DELIVER_MESSAGE;

use std::error::Error;
use std::sync::RwLock;

mod kv;

const SYSTEM_ACTOR: &str = "system";
const CAPABILITY_ID: &str = "wascc:messaging"; // TODO: change this to an appropriate capability ID
const VERSION: &str = env!("CARGO_PKG_VERSION");
const REVISION: u32 = 0; // Typically incremented after each publication in crates or [gantry](https://github.com/wascc/gantry)

#[cfg(not(feature = "static_plugin"))]
capability_provider!(KeyvalueProvider, KeyvalueProvider::new);



pub struct KeyvalueProvider {
    dispatcher: RwLock<Box<dyn Dispatcher>>,
}

impl Default for KeyvalueProvider {
    fn default() -> Self {
        let _ = env_logger::try_init();

        KeyvalueProvider { 
            dispatcher: RwLock::new(Box::new(NullDispatcher::new())),           
        }
    }
}

impl KeyvalueProvider {
    pub fn new() -> Self {
        Self::default()
    }

    fn configure(
        &self,
        _config: CapabilityConfiguration,
    ) -> Result<Vec<u8>, Box<dyn Error>> {        

        // Handle actor binding metadata here...
        // This is typically where you would establish a
        // client or connection to a resource on behalf of
        // an actor

        Ok(vec![])
    }   
    
    fn deconfigure(
        &self,
        _config: CapabilityConfiguration,        
    ) -> Result<Vec<u8>, Box<dyn Error>> {

        // Handle removal of resources claimed by an actor here
        Ok(vec![])
    }

    // Capability providers must provide a descriptor to the host containing metadata and a list of supported operations
    fn get_descriptor(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(serialize(
            CapabilityDescriptor::builder()
                .id(CAPABILITY_ID)
                .name("New Keyvalue Capability Provider") // TODO: change this friendly name
                .long_description("A new Keyvalue capability provider for waSCC actors") // TODO: change this documentation
                .version(VERSION)
                .revision(REVISION)
                .with_operation("SomeOperation", OperationDirection::Both, "Describe the operation here") // TODO: make the operation descriptors match your real interface
                .build()
        )?)
    }
}

impl CapabilityProvider for KeyvalueProvider {    
    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }
    
    // Invoked by host runtime to allow an actor to make use of the capability
    // All providers MUST handle the "configure" message, even if no work will be done
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        trace!("Received host call from {}, operation - {}", actor, op);

        match op {            
            OP_BIND_ACTOR if actor == SYSTEM_ACTOR => self.configure(deserialize(msg)?),
            OP_REMOVE_ACTOR if actor == SYSTEM_ACTOR => self.deconfigure(deserialize(msg)?),
            OP_GET_CAPABILITY_DESCRIPTOR if actor == SYSTEM_ACTOR => self.get_descriptor(),
            "TestOperation" => {
                let lock = self.dispatcher.read().unwrap();
                println!("### 111");
                let msg = BrokerMessage {
                    subject: "11".to_string(),
                    reply_to: "22".to_string(),
                    body: "hello message".as_bytes().to_vec(),
                };
                if let Err(e) = lock.dispatch(actor, OP_DELIVER_MESSAGE, &serialize(msg).unwrap()) {
        			error!("Dispatch failed in test relay {}", e);
        		}
                println!("### 222");
                Ok(Vec::new())
            }
            _ => Err("bad dispatch".into()),
        }
    }
}
