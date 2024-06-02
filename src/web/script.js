const gameField = document.getElementById('gameField');

// Create the 20x20 grid
for (let i = 0; i < 400; i++) {
    const cell = document.createElement('div');
    cell.classList.add('cell');
    gameField.appendChild(cell);
}

document.getElementById('startButton').addEventListener('click', async () => {
    const response = await fetch('/start');
    // const initialGameData = await response.json();
    // updateGameField(initialGameData);
    setInterval(fetchGameData, 1000);
});

async function fetchGameData() {
    try {
        const response = await fetch('http://127.0.0.1:8080/snake');
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        const gameData = await response.json();
        updateGameField(gameData);
    } catch (error) {
        console.error('Failed to fetch game data:', error);
    }
}

function updateGameField(gameData) {
    // Clear the game field
    const cells = document.querySelectorAll('.cell');
    cells.forEach(cell => {
        cell.classList.remove('snake', 'food');
    });

    // Render the snakes for all players
    gameData.players.forEach(player => {
        player.snake_position.forEach(position => {
            const index = position.y * 20 + position.x;
            cells[index].classList.add('snake');
        });
    });

    // Render the food
    const foodIndex = gameData.food.y * 20 + gameData.food.x;
    cells[foodIndex].classList.add('food');
}