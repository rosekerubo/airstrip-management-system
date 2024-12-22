#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use std::collections::HashMap;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Airstrip struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Airstrip {
    id: u64,
    name: String,
    location: String,
    contact: String,
    email: String,
    runway_length: u64, // in meters
    capacity: u64,      // maximum number of planes
    created_at: u64,
}

// Flight struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Flight {
    id: u64,
    airstrip_id: u64,
    flight_number: String,
    destination: String,
    departure_time: u64,
    arrival_time: u64,
    status: String, // "scheduled", "delayed", "completed"
}

// Pilot struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Pilot {
    id: u64,
    name: String,
    license_number: String,
    experience_years: u64,
    contact: String,
    email: String,
}

// PilotSchedule struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct PilotSchedule {
    id: u64,
    pilot_id: u64,
    flight_id: u64,
    start_time: u64,
    end_time: u64,
    status: String, // "scheduled", "completed", "cancelled"
}

// EmergencyProtocol struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EmergencyProtocol {
    id: u64,
    airstrip_id: u64,
    protocol_type: String, // "weather", "technical", "security", "medical"
    description: String,
    contact_numbers: Vec<String>,
    evacuation_routes: Vec<String>,
    created_at: u64,
}

// FuelInventory struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FuelInventory {
    id: u64,
    airstrip_id: u64,
    fuel_type: String,
    quantity: f64,
    unit_price: f64,
    last_updated: u64,
}

// Revenue struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Revenue {
    id: u64,
    airstrip_id: u64,
    source: String, // "landing_fees", "fuel_sales", "parking", "maintenance"
    amount: f64,
    transaction_date: u64,
    description: String,
}

// CapacityStatus struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct CapacityStatus {
    total_capacity: u64,
    current_occupancy: u64,
    scheduled_arrivals: u64,
    scheduled_departures: u64,
    available_slots: u64,
}

// MaintenanceSchedule struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct MaintenanceSchedule {
    id: u64,
    airstrip_id: u64,
    date: u64,
    description: String,
    status: String, // "scheduled", "completed"
}

// Payload structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateAirstripPayload {
    name: String,
    location: String,
    contact: String,
    email: String,
    runway_length: u64,
    capacity: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ScheduleFlightPayload {
    airstrip_id: u64,
    flight_number: String,
    destination: String,
    departure_time: u64,
    arrival_time: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RegisterPilotPayload {
    name: String,
    license_number: String,
    experience_years: u64,
    contact: String,
    email: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ScheduleMaintenancePayload {
    airstrip_id: u64,
    date: u64,
    description: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

// Implementing Storable for Airstrip
impl Storable for Airstrip {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Airstrip {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Flight
impl Storable for Flight {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Flight {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Pilot
impl Storable for Pilot {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Pilot {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for PilotSchedule
impl Storable for PilotSchedule {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for PilotSchedule {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for EmergencyProtocol
impl Storable for EmergencyProtocol {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EmergencyProtocol {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for FuelInventory
impl Storable for FuelInventory {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FuelInventory {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Revenue
impl Storable for Revenue {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Revenue {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for MaintenanceSchedule
impl Storable for MaintenanceSchedule {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for MaintenanceSchedule {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static AIRSTRIPS: RefCell<StableBTreeMap<u64, Airstrip, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10)))
        ));

    static FLIGHTS: RefCell<StableBTreeMap<u64, Flight, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(11)))
        ));

    static PILOTS: RefCell<StableBTreeMap<u64, Pilot, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(12)))
        ));

    static MAINTENANCE_SCHEDULES: RefCell<StableBTreeMap<u64, MaintenanceSchedule, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(13)))
        ));

    static PILOT_SCHEDULES: RefCell<StableBTreeMap<u64, PilotSchedule, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(14)))
        ));

    static EMERGENCY_PROTOCOLS: RefCell<StableBTreeMap<u64, EmergencyProtocol, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(15)))
        ));

    static FUEL_INVENTORIES: RefCell<StableBTreeMap<u64, FuelInventory, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(16)))
        ));

    static REVENUES: RefCell<StableBTreeMap<u64, Revenue, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(17)))
        ));
}

// Functions

// Create Airstrip
#[ic_cdk::update]
fn create_airstrip(payload: CreateAirstripPayload) -> Result<Airstrip, Message> {
    if payload.name.is_empty() || payload.contact.is_empty() || payload.email.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let airstrip_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let airstrip = Airstrip {
        id: airstrip_id,
        name: payload.name,
        location: payload.location,
        contact: payload.contact,
        email: payload.email,
        runway_length: payload.runway_length,
        capacity: payload.capacity,
        created_at: time(),
    };

    AIRSTRIPS.with(|airstrips| {
        airstrips.borrow_mut().insert(airstrip_id, airstrip.clone());
    });

    Ok(airstrip)
}

// Schedule Flight
#[ic_cdk::update]
fn schedule_flight(payload: ScheduleFlightPayload) -> Result<Flight, Message> {
    if payload.flight_number.is_empty() || payload.destination.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let airstrip_exists = AIRSTRIPS.with(|airstrips| airstrips.borrow().contains_key(&payload.airstrip_id));
    if !airstrip_exists {
        return Err(Message::NotFound("Airstrip not found".to_string()));
    }

    let flight_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let flight = Flight {
        id: flight_id,
        airstrip_id: payload.airstrip_id,
        flight_number: payload.flight_number,
        destination: payload.destination,
        departure_time: payload.departure_time,
        arrival_time: payload.arrival_time,
        status: "scheduled".to_string(),
    };

    FLIGHTS.with(|flights| {
        flights.borrow_mut().insert(flight_id, flight.clone());
    });

    Ok(flight)
}

// Register Pilot
#[ic_cdk::update]
fn register_pilot(payload: RegisterPilotPayload) -> Result<Pilot, Message> {
    if payload.name.is_empty() || payload.license_number.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let pilot_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let pilot = Pilot {
        id: pilot_id,
        name: payload.name,
        license_number: payload.license_number,
        experience_years: payload.experience_years,
        contact: payload.contact,
        email: payload.email,
    };

    PILOTS.with(|pilots| {
        pilots.borrow_mut().insert(pilot_id, pilot.clone());
    });

    Ok(pilot)
}

// Schedule Maintenance
#[ic_cdk::update]
fn schedule_maintenance(payload: ScheduleMaintenancePayload) -> Result<MaintenanceSchedule, Message> {
    if payload.description.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let airstrip_exists = AIRSTRIPS.with(|airstrips| airstrips.borrow().contains_key(&payload.airstrip_id));
    if !airstrip_exists {
        return Err(Message::NotFound("Airstrip not found".to_string()));
    }

    let maintenance_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let maintenance = MaintenanceSchedule {
        id: maintenance_id,
        airstrip_id: payload.airstrip_id,
        date: payload.date,
        description: payload.description,
        status: "scheduled".to_string(),
    };

    MAINTENANCE_SCHEDULES.with(|schedules| {
        schedules.borrow_mut().insert(maintenance_id, maintenance.clone());
    });

    Ok(maintenance)
}

// Flight Capacity Management
#[ic_cdk::query]
fn get_capacity_status(airstrip_id: u64) -> Result<CapacityStatus, Message> {
    let airstrip = match AIRSTRIPS.with(|airstrips| airstrips.borrow().get(&airstrip_id)) {
        Some(a) => a,
        None => return Err(Message::NotFound("Airstrip not found".to_string())),
    };

    let current_time = time();
    let mut scheduled_arrivals = 0;
    let mut scheduled_departures = 0;
    let mut current_occupancy = 0;

    FLIGHTS.with(|flights| {
        for (_, flight) in flights.borrow().iter() {
            if flight.airstrip_id == airstrip_id {
                if flight.status == "scheduled" {
                    if flight.arrival_time > current_time {
                        scheduled_arrivals += 1;
                    }
                    if flight.departure_time > current_time {
                        scheduled_departures += 1;
                    }
                } else if flight.status == "arrived" {
                    current_occupancy += 1;
                }
            }
        }
    });

    Ok(CapacityStatus {
        total_capacity: airstrip.capacity,
        current_occupancy,
        scheduled_arrivals,
        scheduled_departures,
        available_slots: airstrip.capacity.saturating_sub(current_occupancy + scheduled_arrivals - scheduled_departures),
    })
}

// Pilot Scheduling
#[ic_cdk::update]
fn schedule_pilot(pilot_id: u64, flight_id: u64, start_time: u64, end_time: u64) -> Result<PilotSchedule, Message> {
    // Verify pilot exists
    let pilot_exists = PILOTS.with(|pilots| pilots.borrow().contains_key(&pilot_id));
    if !pilot_exists {
        return Err(Message::NotFound("Pilot not found".to_string()));
    }

    // Verify flight exists
    let flight_exists = FLIGHTS.with(|flights| flights.borrow().contains_key(&flight_id));
    if !flight_exists {
        return Err(Message::NotFound("Flight not found".to_string()));
    }

    // Check pilot availability
    let is_available = PILOT_SCHEDULES.with(|schedules| {
        schedules.borrow().iter().all(|(_, schedule)| {
            if schedule.pilot_id == pilot_id {
                !(start_time < schedule.end_time && end_time > schedule.start_time)
            } else {
                true
            }
        })
    });

    if !is_available {
        return Err(Message::Error("Pilot is not available for this time slot".to_string()));
    }

    let schedule_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1).unwrap();
        current_value
    });

    let schedule = PilotSchedule {
        id: schedule_id,
        pilot_id,
        flight_id,
        start_time,
        end_time,
        status: "scheduled".to_string(),
    };

    PILOT_SCHEDULES.with(|schedules| {
        schedules.borrow_mut().insert(schedule_id, schedule.clone());
    });

    Ok(schedule)
}

// Emergency Protocols
#[ic_cdk::update]
fn create_emergency_protocol(
    airstrip_id: u64,
    protocol_type: String,
    description: String,
    contact_numbers: Vec<String>,
    evacuation_routes: Vec<String>,
) -> Result<EmergencyProtocol, Message> {
    let protocol_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1).unwrap();
        current_value
    });

    let protocol = EmergencyProtocol {
        id: protocol_id,
        airstrip_id,
        protocol_type,
        description,
        contact_numbers,
        evacuation_routes,
        created_at: time(),
    };

    EMERGENCY_PROTOCOLS.with(|protocols| {
        protocols.borrow_mut().insert(protocol_id, protocol.clone());
    });

    Ok(protocol)
}

// Fuel Management
#[ic_cdk::update]
fn update_fuel_inventory(
    airstrip_id: u64,
    fuel_type: String,
    quantity: f64,
    unit_price: f64,
) -> Result<FuelInventory, Message> {
    let inventory_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1).unwrap();
        current_value
    });

    let inventory = FuelInventory {
        id: inventory_id,
        airstrip_id,
        fuel_type,
        quantity,
        unit_price,
        last_updated: time(),
    };

    FUEL_INVENTORIES.with(|inventories| {
        inventories.borrow_mut().insert(inventory_id, inventory.clone());
    });

    Ok(inventory)
}

// Revenue Tracking
#[ic_cdk::update]
fn record_revenue(
    airstrip_id: u64,
    source: String,
    amount: f64,
    description: String,
) -> Result<Revenue, Message> {
    let revenue_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1).unwrap();
        current_value
    });

    let revenue = Revenue {
        id: revenue_id,
        airstrip_id,
        source,
        amount,
        transaction_date: time(),
        description,
    };

    REVENUES.with(|revenues| {
        revenues.borrow_mut().insert(revenue_id, revenue.clone());
    });

    Ok(revenue)
}

// Revenue Analysis
#[ic_cdk::query]
fn get_revenue_analysis(airstrip_id: u64, start_time: u64, end_time: u64) -> Result<HashMap<String, f64>, Message> {
    let mut analysis = HashMap::new();
    let mut total_revenue = 0.0;

    REVENUES.with(|revenues| {
        for (_, revenue) in revenues.borrow().iter() {
            if revenue.airstrip_id == airstrip_id 
               && revenue.transaction_date >= start_time 
               && revenue.transaction_date <= end_time {
                *analysis.entry(revenue.source.clone()).or_insert(0.0) += revenue.amount;
                total_revenue += revenue.amount;
            }
        }
    });

    analysis.insert("total".to_string(), total_revenue);
    Ok(analysis)
}

// Query functions for new features
#[ic_cdk::query]
fn get_pilot_schedule(pilot_id: u64) -> Vec<PilotSchedule> {
    PILOT_SCHEDULES.with(|schedules| {
        schedules
            .borrow()
            .iter()
            .filter(|(_, schedule)| schedule.pilot_id == pilot_id)
            .map(|(_, schedule)| schedule)
            .collect()
    })
}

#[ic_cdk::query]
fn get_emergency_protocols(airstrip_id: u64) -> Vec<EmergencyProtocol> {
    EMERGENCY_PROTOCOLS.with(|protocols| {
        protocols
            .borrow()
            .iter()
            .filter(|(_, protocol)| protocol.airstrip_id == airstrip_id)
            .map(|(_, protocol)| protocol)
            .collect()
    })
}

#[ic_cdk::query]
fn get_fuel_inventory(airstrip_id: u64) -> Vec<FuelInventory> {
    FUEL_INVENTORIES.with(|inventory| {
        inventory
            .borrow()
            .iter()
            .filter(|(_, inv)| inv.airstrip_id == airstrip_id)
            .map(|(_, inv)| inv)
            .collect()
    })
}

// Exporting the candid interface
ic_cdk::export_candid!();
