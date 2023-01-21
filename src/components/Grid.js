import React from "react";
import { Cell } from "./Cell";

export const Grid = (props) => {
  const { width, height } = props;

  function makeRenderList() {
    let index = 0;
    let gridList = [];
    ///
    for (let i = 0; i < width; i++) {
      let rowList = [];
      for (let j = 0; j < height; j++) {
        index++;
        rowList.push(
          <Cell
            key={j}
            index={index}
          ></Cell>
        );
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
      <p>
        Grid {width}, {height}
      </p>
      {makeRenderList()}
    </div>
  );
};
