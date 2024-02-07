const gameBoard = document.querySelector('#gameboard');
const playerDisplay = document.querySelector('#player');
const infoDisplay = document.querySelector('#info-display');
const socket = new WebSocket('ws://127.0.0.1:8080/ws');

// called when the connection is established
socket.onopen = function () {
	console.log("WebSocket connection established");
	// used to join desired room, just socket.send("/join <anything>")
	socket.send("/join test");
	socket.send("HELLO WORLD");
};

socket.onclose = function () {
	console.log("WebSocket connection closed");
};

const width = 8;


let playerGo = 'Weiss';

playerDisplay.textContent = playerGo;

// Array of all pieces and empty squares
const startPieces = [
    rook, knight, bishop, queen, king, bishop, knight, rook,
    pawn, pawn, pawn, pawn, pawn, pawn, pawn, pawn,
    '', '', '', '', '', '', '', '',
    '', '', '', '', '', '', '', '',
    '', '', '', '', '', '', '', '',
    '', '', '', '', '', '', '', '',
    pawn, pawn, pawn, pawn, pawn, pawn, pawn, pawn,
    rook, knight, bishop, queen, king, bishop, knight, rook
];

// Creates Board and add pieces
function createBoard() {
    startPieces.forEach((startPiece, i) => {
        const square = document.createElement('div');

        square.classList.add('square');
        square.setAttribute('square-id', i);
        square.innerHTML = startPiece;
        square.firstChild?.setAttribute('draggable', 'true');

        // Zero-based: first row = 0, second row = 1, etc...
        const row = Math.floor(i / 8);

        // Determine the color of Squares
        if (row % 2 === 0) {
            square.classList.add(i % 2 === 0 ? 'Weiss-square' : 'Schwarz-square');
            } else {
            square.classList.add(i % 2 === 0 ? 'Schwarz-square' : 'Weiss-square');
            }

        // Determine the color of the pieces
        if (i <= 15) {
            square.firstChild.firstChild.classList.add('Schwarz-piece');
        } else if (i >= 48) {

            square.firstChild.firstChild.classList.add('Weiss-piece');
        }

        gameBoard.append(square);
    });
}


createBoard();



// The position on the board the piece starts from
let startPositionId = -1;
let draggedElement;
let droppedElement;

const allSquares = document.querySelectorAll('.square');

// Add event listener
allSquares.forEach(square => {
    square.addEventListener('dragstart', dragStart);
    square.addEventListener('dragover', dragOver);
    square.addEventListener('drop', dragDrop);
});


function dragStart(e) {
    draggedElement = e.target;
    startPositionId = draggedElement.parentNode.getAttribute('square-id');
}


function dragOver(e) {
    //
    e.preventDefault();
    droppedElement = e.target
}


function dragDrop(e) {
    //
    e.stopPropagation();

    const correctGo = draggedElement.firstChild.classList.contains(playerGo + '-piece');
    // Ternary operator
    console.log(e.target)
    const opponentGo = playerGo === 'Schwarz' ? 'Weiss' : 'Schwarz';
    const taken = e.target.classList.contains('piece');
    const takenByOpponent = e.target.firstChild?.classList.contains(opponentGo + '-piece');
    if (correctGo) {
        if (isValidMove(e.target, taken, takenByOpponent)) {
            //console.log(e.target)
            const endPositionId = e.target.getAttribute('square-id');
            const moveString = `${getPositionAlgebraic(startPositionId)} to ${getPositionAlgebraic(endPositionId)}`;
            
            
            // This will immediately reset the info display when a valid move is made before the notification reset timer clears the last notification
            notifyPlayer('', false);
            if (!taken) {
                e.target.append(draggedElement);
                // Only change players if the game is still ongoing
                // Output moves to the console and WebSocket
                console.log(moveString);
                socket.send(moveString);
                if (!checkWin()) changePlayer();
            } else if (takenByOpponent) {
                e.target.setAttribute('square-id', droppedElement.parentNode.getAttribute('square-id'))
                const endPositionId = e.target.getAttribute('square-id');
                const takenString = `${getPositionAlgebraic(startPositionId)} hit ${getPositionAlgebraic(endPositionId)}`;
                //console.log(startPositionId, endPositionId)
                //Ouput moves that took another figure to console and WebSocket
                console.log(endPositionId)
                console.log(takenString);
                socket.send(takenString);
                document.getElementById(`${playerGo}-captures`).innerHTML += `<div class="captured-piece">${e.target.innerHTML}</div>`;
                e.target.parentNode.append(draggedElement);
                e.target.remove();
                
                // Only change players if the game is still ongoing
                if (!checkWin()) changePlayer();
            } else notifyPlayer('Du kannst dort nicht hin!');
        }
        else notifyPlayer('Du kannst dort nicht hin!');
    }
}

function getPositionAlgebraic(positionId) {
    // Convert positionId to algebraic notation (e.g., 0 to a8, 1 to b8, etc.)
    const file = String.fromCharCode('a'.charCodeAt(0) + (positionId % 8));
    const rank = 8 - Math.floor(positionId / 8); // Subtract the rank from 8
    return `${file}${rank}`;
}


// DEBUG
//console.log(getPositionAlgebraic(63));

function notifyPlayer(message, useTimer = true) {
    infoDisplay.textContent = message;
    if (useTimer) setTimeout(() => { infoDisplay.textContent = '' }, 2000);
}


function changePlayer() {
    playerGo = playerGo === 'Weiss' ? 'Schwarz' : 'Weiss';
    playerDisplay.textContent = playerGo;
}


function isValidMove(target, taken, takenByOpponent) {
    const targetId = Number(target.getAttribute('square-id') || target.parentNode.getAttribute('square-id'));
    const startId = Number(startPositionId);
    const idInterval = Math.abs(targetId - startId);
    const piece = draggedElement.id;

    let startRow = Math.floor(startId / width);
    let startCol = startId % width;
    let targetRow = Math.floor(targetId / width);
    let targetCol = targetId % width;

    // How far apart are the rows and how far apart are the columns
    const rowInterval = targetRow - startRow;
    const colInterval = targetCol - startCol;

    const validMoves = {
        'pawn': () => {
            let direction = 1;
            // Flip the rows depending on who's playing. 
            if (playerGo === 'Weiss') {
                startRow = width - 1 - startRow;
                targetRow = width - 1 - targetRow;
                direction = -1;
            }
            // Check if the pawn's movement is blocked by any piece
            const blockedByPiece = Boolean(document.querySelector(`[square-id="${startId + direction * width}"]`).firstChild);

            return targetRow > startRow && ((!taken && !blockedByPiece && startRow === 1 && idInterval === 2 * width) || (!taken && idInterval === width) || (takenByOpponent && (idInterval === width - 1 || idInterval === width + 1)));
        },
        'rook': () => {
            // Successful vertical movement or horizontal movement
            if ((rowInterval !== 0 && colInterval === 0) || (rowInterval === 0 && colInterval !== 0)) {
                // Check if the rook's movement is blocked by any piece
                for (let i = Math.abs(rowInterval ? rowInterval : colInterval) - 1; i > 0; --i) {
                    const id = rowInterval ? startId + Math.sign(rowInterval) * i * width : startId + Math.sign(colInterval) * i;
                    if (Boolean(document.querySelector(`[square-id="${id}"]`).firstChild)) {
                        return false;
                    }
                }
                return true;
            }
            return false;
        },
        'bishop': () => {
            // Successful diagonal movement
            if (Math.abs(rowInterval) === Math.abs(colInterval) && rowInterval !== 0) {
                // Check if the bishop's movement is blocked by any piece
                for (let i = Math.abs(rowInterval) - 1; i > 0; --i) {
                    if (Boolean(document.querySelector(`[square-id="${startId + Math.sign(rowInterval) * i * width + Math.sign(colInterval) * i
                        }"]`).firstChild)) {
                        return false;
                    }
                }
                return true;
            }
            return false;
        },
        'knight': () => {
            // Two steps up or down, one step right or left - Two steps right or left, one step up or down
            return (Math.abs(rowInterval) === 2 && Math.abs(colInterval) === 1) || (Math.abs(colInterval) === 2 && Math.abs(rowInterval) === 1);
        },
        'queen': () => {
            // A queen is simply just a rook and a bishop at the same time
            // return this.rook() || this.bishop();
            return (validMoves['rook']() || validMoves['bishop']());
        },
        'king': () => {
            // King moves one step anywhere
            return (idInterval === width || idInterval === width - 1 || idInterval === width + 1 || idInterval === 1);
        }
    }

    return validMoves[piece]();
}

function checkWin() {
    const kings = document.querySelectorAll('#gameboard #king');

    // If there is one less king piece then the current player wins: The current player only changes when this function returns false
    if (kings.length < 2) {
        notifyPlayer(`${playerGo}er Spieler gewinnt!`, false);
        playerDisplay.parentElement.textContent = '';
        console.log(`${playerGo} won`);
        socket.send(`${playerGo} won`);
        playerGo = '';
        
        // Make all the remaining pieces non-draggable so the game kind of ends
        document.querySelectorAll('.piece').forEach(piece => {
            piece.setAttribute('draggable', false);
        });

        return true;
    }

    return false;
}
