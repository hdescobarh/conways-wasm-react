import React, { useContext } from "react";
import { SimulationContext } from "../contexts/Simulation";

export const Start = () => {
  const { handleClick, run } = useContext(SimulationContext);
  const baseStyle = "rounded-full py-2 px-4 font-bold text-white bg-amber-500";
  return (
    <div className="flex items-center justify-center bg-inherit">
      <button
        className={`${baseStyle} ${
          run ? "hover:bg-red-600" : "hover:bg-green-600"
        }`}
        onClick={handleClick}
      >
        {run ? "Stop" : "Start"}
      </button>
    </div>
  );
};
