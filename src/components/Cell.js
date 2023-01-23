import React, { memo } from "react";

export const Cell = memo((props) => {
  const { alive } = props;

  const cellStyle =
    "flex h-2 w-2 border border-slate-300 justify-center items-center";

  return (
    <div
      className={`${cellStyle} ${alive ? " bg-rose-500" : " bg-slate-400"}`}
    ></div>
  );
});
