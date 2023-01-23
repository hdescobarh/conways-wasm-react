import React, { Fragment, useContext, useEffect } from "react";
import { Cell } from "./Cell";
import { SimulationContext } from "../contexts/Simulation";

export const Grid = (props) => {
  const { width, height } = props;
  const { currentSimulation, setGeneration, run, generation } =
    useContext(SimulationContext);

  const setCellComponent = (index) => {
    if (currentSimulation) {
      const alive = currentSimulation.get_cell_value(index);
      return <Cell alive={alive}></Cell>;
    }
  };

  const renderCells = () => {
    let cellsToRender = new Array((width - 1) * (height - 1));
    let currentIndex = 0;

    for (let i = 0; i < width; i++) {
      let rowList = new Array(height - 1);
      for (let j = 0; j < height; j++) {
        rowList[j] = (
          <Fragment key={j}>{setCellComponent(currentIndex)}</Fragment>
        );
        currentIndex++;
      }
      cellsToRender[i] = (
        <div
          key={i}
          className="flex"
        >
          {rowList}
        </div>
      );
    }
    return cellsToRender;
  };

  useEffect(() => {
    let interval = null;
    if (run) {
      interval = setInterval(() => {
        let newGeneration = currentSimulation.time_step();
        setGeneration(newGeneration);
      }, 200);
    }
    return () => {
      if (interval) {
        clearInterval(interval);
      }
    };
  }, [run]);

  return (
    <div className="flex flex-col items-center bg-inherit pb-8">
      <h2>
        Grid {width}, {height}. Status: {run ? "running" : "stopped"}. Gen:
        {generation}
      </h2>
      {renderCells()}
    </div>
  );
};
