import { Accessor, createContext, ParentProps, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import { Heat, NewRaceDto, Pilot, Race, RaceEvent } from "./models";
import { invoke } from "@tauri-apps/api/tauri";

export const initialState: State = {
  raceEvents: [],
  selectedRaceEventId: 0,
  pilots: [],
  races: []
}

export interface State {
  raceEvents: RaceEvent[];
  selectedRaceEventId: number;
  pilots: Pilot[];
  races: Race[];
}

function stateProviderFactory(initialState: State) {
  const [state, setState] = createStore<State>(initialState);

  const methods = {
    raceEvents: {
      init() {
        invoke<{race_events: RaceEvent[]}>("init").then((initState) => {
          setState("raceEvents", initState.race_events);
        })
      },
      addOne(eventName: string) {
        invoke<RaceEvent>("create_race_event", { newRaceEventDto: { name: eventName } })
          .then((raceEvent) => {
            console.log(raceEvent);
            setState("raceEvents", raceEvents => ([...raceEvents, raceEvent]));
          })
      },
      selectOne(id: number) {
        setState(oldState => ({ ...oldState, selectedRaceEventId: id }))
      }
    },
    pilots: {
      addOne(pilotName: string) {
        invoke<Pilot>("set_pilot", { pilot: { name: pilotName, raceEventId: state.selectedRaceEventId } })
          .then((pilot) => {
            setState("pilots", pilots => ([...pilots, pilot]));
          }).catch(console.log);
      },
    },
    races: {
      addOne(heats: Heat[]) {
        invoke<Race>('add_race', { newRaceDto: { name: "Heat name", heats, raceEventId: state.selectedRaceEventId }satisfies NewRaceDto })
          .then((newRace: Race) => {
            setState("races", oldRaces => ([...oldRaces, newRace]))
          });
      }
    }
  }

  return [
    state,
    methods
  ] as [typeof state, typeof methods];
}

const stateProvider = stateProviderFactory(initialState);

const StateContext = createContext(stateProvider);
export function StateProvider(props: ParentProps<{ initialState: State }>) {
  return (
    <StateContext.Provider value={stateProvider}>
      {props.children}
    </StateContext.Provider>
  )
}

export function useAppState() {
  return useContext(StateContext);
}
