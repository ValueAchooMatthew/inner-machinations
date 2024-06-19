import type { AutomataInformation, Coordinate, TauriGeneratedAutomataInformation } from "./interfaces";

export const convertCoordinateToString = (coordinate: Coordinate): string => {
  return coordinate.x + "," + coordinate.y;
};

// export const parseTauriReponse = (tauri_response: TauriGeneratedAutomataInformation): AutomataInformation {




// }

