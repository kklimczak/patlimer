export interface Pilot {
    id: number;
    name: string;
    raceEventId: number;
}

export interface Heat {
    no: number;
    channel: string;
    pilot_id: number;
}

export type RaceStatus = "New" | "InProgress" | "Interrupted" | "Finished";

export interface Race {
    id: string;
    name: string;
    status: RaceStatus;
    heats: Heat[];
    raceEventId: number;
}

export interface NewHeatDto {
    no: number;
    pilot_id: number;
    channel: string;
}

export interface NewRaceDto {
    name: string;
    heats: NewHeatDto[];
    race_event_id: number;
}

export interface Slot {
    channel: string;
    pilot_id: number;
}

export interface RaceEvent {
    id: number;
    name: string;
    race_event_type: "Local" | "Cloud";
    created_at: string;
}

export interface NewRaceEventDto {
    name: string;
}

export interface RaceDetailsDto {
    pilots: Pilot[];
    races: Race[];
}
