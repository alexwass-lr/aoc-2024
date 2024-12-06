const gridContainer = document.getElementById("grid");
var grid = new Map();
var start = [0, 0];

// load the puzzle and create elements for each row and column
document
    .getElementById("upload")
    .addEventListener("change", function (event) {
        const file = event.target.files[0];
        if (file) {
            const reader = new FileReader();
            reader.onload = function (e) {
                event.target.remove();

                const content = e.target.result;
                renderMatrix(content);
                startPatrol();
            };
            reader.readAsText(file);
        }
    });

function renderMatrix(content) {
    // get the rows
    const rows = content.trim().split("\n");

    // set the grid columns
    const numCols = rows[0].length;
    gridContainer.style.gridTemplateColumns = `repeat(${numCols}, auto)`;

    rows.forEach((row, i) => {
        // add the row to the grid
        grid.set(i, new Map(row.split("").entries()));
        // then render the row
        [...row].forEach((cell) => {
            const cellElement = document.createElement("div");
            if (cell === "#") {
                cellElement.classList.add("obstacle");
            }
            if (cell === "^") {
                start = [[...row].indexOf(cell), i];
                cellElement.id = "current";
            }
            cellElement.classList.add("cell");
            gridContainer.appendChild(cellElement);
        });
    });
}

async function startPatrol() {
    // initial state
    var pos = start;
    var direction = "up";
    var steps = 1;
    var exited = false;

    while (!exited) {
        const [x, y] = getNextStep(pos, direction);

        // the guard has left the grid
        if (x === null || y === null) {
            exited = true;
            continue;
        }

        const currentNode = gridContainer.querySelector("#current");
        // the nodes are in one long list so we need to calculate the index
        const nextNode = gridContainer.querySelector(`.cell:nth-child(${grid.size * y + x + 1})`);

        if (nextNode.classList.contains("obstacle")) {
            direction = turn(direction);
        } else {
            // update the guards position
            currentNode.id = "";
            currentNode.classList.add('path');
            nextNode.id = "current";
            pos = [x, y];

            // don't count existing positions
            if (!nextNode.classList.contains("path")) {
                steps++;
            }

            // render the steps
            await new Promise(r => setTimeout(r, 5));
        }
    }

    alert(`Guard left after ${steps} steps`);
}

function getNextStep(pos, direction) {
    let [x, y] = pos;

    switch (direction) {
        case "up":
            y--;
            break;
        case "right":
            x++;
            break;
        case "down":
            y++;
            break;
        case "left":
            x--;
            break;
    }

    return grid.get(y)?.get(x) ? [x, y] : [null, null];
}

function turn(direction) {
    switch (direction) {
        case "up":
            return "right";
        case "right":
            return "down";
        case "down":
            return "left";
        case "left":
            return "up";
    }
}