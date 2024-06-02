const gameField = document.getElementById('gameField');
const startButton = document.getElementById('startButton');

let gameInterval;

// Create the 20x20 grid
for (let i = 0; i < 400; i++) {
    const cell = document.createElement('div');
    cell.classList.add('cell');
    gameField.appendChild(cell);
}

startButton.addEventListener('click', async () => {
    startButton.disabled = true;
    try {
        const response = await fetch('/start');
        if (!response.ok) {
            throw new Error('Failed to start the game');
        }
        fetchGameData(); 
        gameInterval = setInterval(fetchGameData, 200); 
    } catch (error) {
        console.error('Failed to start the game:', error);
        startButton.disabled = false;
    }
});

document.addEventListener('keydown', async (event) => {
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
});

async function fetchGameData() {
    try {
        const response = await fetch('/snake');
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        const gameData = await response.json();
        updateGameField(gameData);
    } catch (error) {
        console.error('Failed to fetch game data:', error);
        clearInterval(gameInterval);
        startButton.disabled = false;
    }
}

function updateGameField(gameData) {
    // Clear the game field
    const cells = document.querySelectorAll('.cell');
    cells.forEach(cell => {
        cell.classList.remove('snake', 'food');
    });

    // Render the snakes of all players
    gameData.players.forEach(player => {
        player.snake.body.forEach(position => {
            const index = position.y * 20 + position.x;
            cells[index].classList.add('snake');
        });
    });

    // Render the food
    const foodIndex = gameData.food.y * 20 + gameData.food.x;
    cells[foodIndex].classList.add('food');
}