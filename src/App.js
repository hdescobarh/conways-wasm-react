import React from "react";
import { Grid } from "./components/Grid";

class App extends React.Component {
  // when simulation number changes, trigger each cell to check its state
  // if does not change, stop

  render() {
    const { name } = this.props;
    return (
      <>
        <h1 className="bg-black text-4xl text-white">Hello {name}!</h1>
        <Grid
          width="5"
          height="5"
        />
      </>
    );
  }
}

export default App;
