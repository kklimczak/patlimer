import { Accessor, createContext, ParentProps, useContext } from "solid-js";
import { createStore } from "solid-js/store";
import {Heat, NewHeatDto, NewRaceDto, Pilot, Race, RaceDetailsDto, RaceEvent} from "./models";
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
      addOne(eventName: string) {
        invoke<RaceEvent>("create_race_event", { newRaceEventDto: { name: eventName } })
          .then((raceEvent) => {
            console.log(raceEvent);
            setState("raceEvents", raceEvents => ([...raceEvents, raceEvent]));
          })
      },
      selectOne(id: number) {
        setState(oldState => ({ ...oldState, selectedRaceEventId: id }));
        methods.raceEvents.loadRaceEventDetails(id);
      },
      clearSelection() {
        setState(oldState => ({...oldState, selectedRaceEventId: 0}))
      },
      removeOne(id: number) {
        invoke<unknown>('remove_race_event', {raceEventId: id})
            .then(() => {
              setState("raceEvents", raceEvents => (raceEvents.filter(raceEvent => raceEvent.id !== id)))
            });
      },
      loadRaceEventDetails(id: number) {
        invoke<RaceDetailsDto>('find_race_event_details', {raceEventId: id})
            .then(({pilots, races}) => {
              setState(oldState => ({...oldState, pilots, races}));
            })
      }
    },
    pilots: {
      addOne(pilotName: string) {
        invoke<Pilot>("set_pilot", { newPilotDto: { name: pilotName, race_event_id: state.selectedRaceEventId } })
          .then((pilot) => {
            setState("pilots", pilots => ([...pilots, pilot]));
          }).catch(console.log);
      },
    },
    races: {
      addOne(heats: NewHeatDto[]) {
        invoke<Race>('add_race', { newRaceDto: { name: "Heat name", heats, race_event_id: state.selectedRaceEventId }satisfies NewRaceDto })
          .then((newRace: Race) => {
            setState("races", oldRaces => ([...oldRaces, newRace]))
          });
      }
    }
  }

  invoke<{race_events: RaceEvent[]}>("init").then((initState) => {
    setState("raceEvents", initState.race_events);
  });

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
