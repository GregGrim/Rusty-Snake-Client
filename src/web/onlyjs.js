const gameFieldContainer = document.getElementById('gameField');
const startButton = document.getElementById('startButton');

let gameInterval;
let keydownListener;
let playerID;
let ws;

const gridSize = 20;
let gameField = [];

// Create the 20x20 grid and initialize gameField as a 2D array
for (let y = 0; y < gridSize; y++) {
    const row = [];
    for (let x = 0; x < gridSize; x++) {
        const cell = document.createElement('div');
        cell.classList.add('cell');
        gameFieldContainer.appendChild(cell);
        row.push(cell);
    }
    gameField.push(row);
}
window.onload = () => {
    ws = new WebSocket('wss://0.0.0.0:3000');

    ws.onopen = () => {
        console.log('WebSocket connected');
        // Request a player ID from the server
        ws.send(JSON.stringify({ action: "player_connected" }));
    };

    ws.onmessage = (event) => {
        const message = JSON.parse(event.data);
        console.log(message)

        if (typeof message === 'string') {
            playerID = message;
            console.log('Received player ID:', playerID);
        } else {
            const gameData = message;
            console.log('Received game data:', gameData);

            console.log("players: ", gameData.players);
            //console.log("playerID: ", playerID);
        
            const player = gameData.players.find((player) => player.player_id == playerID);
            if (player && player.game_over) {

                document.removeEventListener('keydown', keydownListener);
                startButton.disabled = false;
                clearGameField();
                alert(`Game Over. Your score: ${player.score}`);
            } else {
                console.log("updating game field")
                updateGameField(gameData);
            }
        }
    };
    ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        startButton.disabled = false;
    };

    ws.onclose = () => {
        // const action = { action: "player_disconnected" };
        // ws.send(JSON.stringify(action));
        console.log('WebSocket closed');
        startButton.disabled = false;
    };
}

startButton.addEventListener('click', async () => {
    startButton.disabled = true;
    
    keydownListener = handleKeydown.bind(null, ws);
    document.addEventListener('keydown', keydownListener);

    const action = { action: "player_started_game", player_id: playerID };
    ws.send(JSON.stringify(action));
});

function handleKeydown(ws, event) {
    let direction;
    switch (event.key) {
        case 'ArrowUp':
            direction = 'Up';
            break;
        case 'ArrowDown':
            direction = 'Down';
            break;
        case 'ArrowLeft':
            direction = 'Left';
            break;
        case 'ArrowRight':
            direction = 'Right';
            break;
        default:
            return; // Exit this handler for other keys
    }

    const action = { action: "player_changed_direction", player_id: playerID, direction: direction };
    ws.send(JSON.stringify(action));
}

function updateGameField(gameData) {
    clearGameField();
    // Render the snakes of all players
    gameData.players.forEach(player => {
        player.snake.body.forEach(position => {
            gameField[position.y][position.x].classList.add('snake');
        });
    });

    // Render the food
    const foodPosition = gameData.food;
    gameField[foodPosition.y][foodPosition.x].classList.add('food');
}

function clearGameField() {
    for (let y = 0; y < gridSize; y++) {
        for (let x = 0; x < gridSize; x++) {
            gameField[y][x].classList.remove('snake', 'food');
        }
    }
}

