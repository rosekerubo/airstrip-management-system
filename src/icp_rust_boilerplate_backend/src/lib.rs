#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

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

// Exporting the candid interface
ic_cdk::export_candid!();
