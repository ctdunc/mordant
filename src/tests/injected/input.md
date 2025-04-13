# test doc

some text
 
```{python}
from dash import clientside_callback, Input, Output
gridid = "grid"
clientside_callback(
    """
(id) => {
  dash_ag_grid.getApiAsync(id).then((api) => {
    api.addEventListener("cellFocused", (params) => {
      console.log(params);
    });
  });
  return dash_clientside.no_update;
};
    """,
    Output(gridid, "id"),
    Input(gridid, "id"),
)
```

another paragraph
