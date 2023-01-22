import React, { useContext } from "react";
import { SimulationContext } from "../contexts/Simulation";

export const Start = () => {
  const { handleClick } = useContext(SimulationContext);
  return <button onClick={handleClick}>Start</button>;
};
