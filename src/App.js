import React, { useContext, Fragment } from "react";
import { Grid } from "./components/Grid";
import { Start } from "./components/Start";
import { SimulationContext } from "./contexts/Simulation";

const App = (props) => {
  const { name } = props;
  const { height, width } = useContext(SimulationContext);
  return (
    <div className="space-y-1 bg-slate-100">
      <h1 className=" bg-neutral-800 p-2 text-4xl text-white">{name}!</h1>
      <Start />
      <Grid
        width={height}
        height={width}
      />
    </div>
  );
};

export default App;
