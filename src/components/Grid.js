import React, { Fragment, useContext } from "react";
import { Cell } from "./Cell";
import { SimulationContext } from "../contexts/Simulation";

export const Grid = (props) => {
  const { width, height } = props;
  const { currentSimulation, generation } = useContext(SimulationContext);

  function renderCell(index) {
    let alive;
    if (currentSimulation) {
      alive = currentSimulation.get_cell_value(index);
    } else {
      alive = false;
    }

    return (
      <Cell
        index={index}
        alive={alive}
      ></Cell>
    );
  }

  function makeRenderList() {
    let index = 0;
    let gridList = [];
    ///
    for (let i = 0; i < width; i++) {
      let rowList = [];
      for (let j = 0; j < height; j++) {
        rowList.push(<Fragment key={j}>{renderCell(index)}</Fragment>);
        index++;
      }
      gridList.push(
        <div
          key={i}
          className="flex"
        >
          {rowList}
        </div>
      );
    }
    return gridList;
  }

  return (
    <div className="mb-8 flex flex-col items-center">
      <h2>
        Grid {width}, {height}. Gen: {generation}
      </h2>
      {makeRenderList()}
    </div>
  );
};
