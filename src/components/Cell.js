import React from "react";

export const Cell = (props) => {
  const { index, key } = props;
  return (
    <div className="flex h-10 w-10 items-center border border-sky-500 text-center text-xs">
      Cell {index}
    </div>
  );
};
