const gameFieldContainer = document.getElementById('gameField');
const startButton = document.getElementById('startButton');

let gameInterval;
let keydownListener;
let playerID;

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

startButton.addEventListener('click', async () => {
    startButton.disabled = true;
    try {
        const response = await fetch('/start');
        if (!response.ok) {
            throw new Error('Failed to start the game');
        }

        playerID = await response.json();

        keydownListener = handleKeydown.bind(null);
        document.addEventListener('keydown', keydownListener);

        fetchGameData(); 
        gameInterval = setInterval(fetchGameData, 200); 
    } catch (error) {
        console.error('Failed to start the game:', error);
        startButton.disabled = false;
    }
});

async function handleKeydown(event) {
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

    try {
        await fetch('/change_direction', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ direction }),
        });
    } catch (error) {
        console.error('Failed to change direction:', error);
    }
}

async function fetchGameData() {
    try {
        const response = await fetch('/snake');
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        const gameData = await response.json();

        console.log("players: ", gameData);
        //console.log("playerID: ", playerID);
        
        const player = gameData.players.find((player) => player.player_id == playerID);
        if (player.game_over) {
            //clearInterval(gameInterval);
            document.removeEventListener('keydown', keydownListener);
            startButton.disabled = false;
            clearGameField();
            alert(`Game Over. Your score: ${player.score}`);
        } else{
            updateGameField(gameData);
        }
    } catch (error) {
        console.error('Failed to fetch game data:', error);
        clearInterval(gameInterval);
        startButton.disabled = false;
    }
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