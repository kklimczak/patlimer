export interface Pilot {
    name: string;
    raceEventId: string;
}

export interface Heat {
    no: number;
    channel: string;
    pilot: Pilot;
}

export type RaceStatus = "New" | "InProgress" | "Interrupted" | "Finished";

export interface Race {
    id: string;
    name: string;
    status: RaceStatus;
    heats: Heat[];
    raceEventId: string;
}

export interface NewRaceDto {
    name: string;
    heats: Heat[];
    raceEventId: string;
}

export interface Slot {
    channel: string;
    pilot: Pilot;
}

export interface RaceEvent {
    _id: string;
    name: string;
    race_event_type: "Local" | "Cloud";
    created_at: string;
}

export interface NewRaceEventDto {
    name: string;
}
