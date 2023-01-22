import React, { useContext, useState, useEffect } from "react";
import { SimulationContext } from "../contexts/Simulation";

export const Cell = (props) => {
  const { index, alive } = props;

  const cellStyle =
    "flex h-2 w-2 border border-sky-300 justify-center items-center";

  // Cell {index},{alive ? "T" : "F"}  // text-xs
  return (
    <div
      className={`${cellStyle} ${alive ? " bg-red-500" : " bg-slate-50"}`}
    ></div>
  );
};
