// https://medium.com/nerd-for-tech/get-global-variables-in-react-js-490cf68f2a73
import React, { createContext, useState, useEffect } from "react";
import init, { Simulation } from "conways-wasm";

export const SimulationContext = createContext();
const SimulationContextProvider = (props) => {
  const [currentSimulation, setCurrentSimulation] = useState(null);
  const [generation, setGeneration] = useState(0);
  const [run, setRun] = useState(false);
  const [width, setWidth] = useState(100);
  const [height, setHeight] = useState(75);

  function makeExample() {
    let array = new Array(7500);
    array.fill(false);
    let start = 25 * width + 40;
    for (let coordinate of [
      [2, 1],
      [3, 2],
      [3, 3],
      [2, 3],
      [1, 3],
      [4, 12],
      [5, 10],
      [5, 11],
      [6, 11],
      [6, 12],
    ]) {
      let index = start + coordinate[0] * width + coordinate[1];
      array[index] = true;
    }
    return array;
  }

  useEffect(() => {
    init().then(() => {
      let space_init = makeExample();
      let startSimulation = Simulation.new();
      startSimulation.initialize_universe(height, width, space_init);
      setCurrentSimulation(startSimulation);
    });
  }, []);

  const handleClick = () => {
    if (currentSimulation) {
      setRun(!run);
    }
  };

  return (
    <SimulationContext.Provider
      value={{
        width,
        height,
        currentSimulation,
        generation,
        handleClick,
        run,
        setGeneration,
      }}
    >
      {props.children}
    </SimulationContext.Provider>
  );
};
export default SimulationContextProvider;
