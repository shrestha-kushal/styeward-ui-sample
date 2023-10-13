export function create_grid(ag_grid_data, grid_div_id) {
  if (navigator.userAgent.match(/Android/i)
    || navigator.userAgent.match(/webOS/i)
    || navigator.userAgent.match(/iPhone/i)
    || navigator.userAgent.match(/iPad/i)
    || navigator.userAgent.match(/iPod/i)
    || navigator.userAgent.match(/BlackBerry/i)
    || navigator.userAgent.match(/Windows Phone/i)) {
      // set up grid properties including providing data
      const gridOptions = {
        columnDefs: ag_grid_data.col_defs,
        domLayout: "autoHeight",
        animateRows: true,
        defaultColDef: {
          flex: 1,
          minWidth: 40,
          filter: true,
          resizable: true,
          wrapText: true,
          sortable: true,
          enableRowGroup: true,
        },
        rowHeight: 20,
        headerHeight: 30,
        alwaysShowHorizontalScroll: true,
        alwaysShowVerticalScroll: true,
        rowGroupPanelShow: "always",
        groupDisplayType: 'groupRows',
        suppressDragLeaveHidesColumns: true,
        rowData: ag_grid_data.row_data,
      };

      // setup the grid after the page has finished loading
      var gridDiv = document.querySelector("#".concat(grid_div_id));
      new agGrid.Grid(gridDiv, gridOptions);
    } else {
      // set up grid properties including providing data
      const gridOptions = {
        columnDefs: ag_grid_data.col_defs,
        domLayout: "normal",
        animateRows: true,
        rowSelection: "multiple",
        defaultColDef: {
          flex: 1,
          minWidth: 100,
          filter: true,
          resizable: true,
          wrapText: true,
          sortable: true,
          enableRowGroup: true,
        },
        rowHeight: 30,
        headerHeight: 40,
        alwaysShowHorizontalScroll: true,
        rowSelection: "multiple",
        enableCellTextSelection: true,
        columnHoverHighlight: true,
        rowDragManaged: true,
        rowDragEntireRow: true,
        rowDragMultiRow: true,
        rowGroupPanelShow: "always",
        groupDisplayType: 'groupRows',
        suppressDragLeaveHidesColumns: true,
        rowData: ag_grid_data.row_data,
      };

      // setup the grid after the page has finished loading
      var gridDiv = document.querySelector("#".concat(grid_div_id));
      new agGrid.Grid(gridDiv, gridOptions);
    }

  

  // return result
  return true;
}

