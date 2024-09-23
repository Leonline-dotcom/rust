


const main = document.getElementById('main');
const searchform =  document.getElementById('searchForm');
const search = document.getElementById('search');
const form = document.querySelector('.movie form');
const overview = document.querySelector('.overview');

const BASE_IMG_URL ="https://image.tmdb.org/t/p/w500";

function showForm() {
    const form = document.querySelector('.movie form');
    const overview = document.querySelector('.overview');
    console.log("clicked");
    form.style.display = 'block';
    overview.style.display ='none';
}   

document.querySelectorAll('.horizontalContainer button').forEach(button => {
    button.addEventListener('click', () => {
        button.classList.toggle('clicked');
    });
});


get_movies();
async function get_movies(){
    try {
        const response = await fetch('/movies');
        const result = await response.json();
        console.log(result);  // Output the fetched data
        showMovies(result.results);

    } catch (error) {
        console.error('Error:', error);
    }
    
}

async function showMovies(data) {
    main.innerHTML = '';
    data.forEach(movie => {
        const { title, poster_path, vote_average, overview } = movie;
        const movieEl = document.createElement('div');
        movieEl.classList.add('movie');

        movieEl.innerHTML = `
            <img src="${BASE_IMG_URL + poster_path}" alt="${title}">
            <div class="movie-info">
                <h3>${title}</h3>
                <span class="${getColor(vote_average)}">${vote_average.toFixed(1)}</span>
            </div>
            <div class="overview">
                <h3>Summary</h3>
                ${overview}
            </div>
            <form class="reviewForm">
                <input type="text" placeholder="Enter Name" class="username" required>
                <textarea class="review" placeholder="How was the movie?" required></textarea>
                <button type="submit" class="submit_review_btn">Submit Review</button>
            </form>
        `;

        // Add click event to movieEl to trigger showForm
        movieEl.addEventListener('click', () => {
            showForm(movieEl);
        });

        // Check if submit button exists before adding listener
        const submitBtn = movieEl.querySelector('.submit_review_btn');
        if (submitBtn) {
            submitBtn.addEventListener('click', async function (event) {
                event.preventDefault();
                const usernameInput = movieEl.querySelector('.username');
                const reviewInput = movieEl.querySelector('.review');

                // Check if username and review elements exist before using them
                if (usernameInput && reviewInput) {
                    const username = usernameInput.value;
                    const review = reviewInput.value;

                    const response = await fetch('/submit_review', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ username, review })
                    });

                    const result = await response.text();
                    document.getElementById('responseMessage').innerText = result;

                    // Reset the form after submission
                    movieEl.querySelector('form').reset();
                    movieEl.querySelector('form').style.display = 'none';
                    movieEl.querySelector('.overview').style.display = 'block';
                }
            });
        }

        main.appendChild(movieEl);
    });
}

document.getElementById('submit_review_btn').addEventListener('click', async function(event) {
    event.preventDefault();
    const username = document.getElementById('username').value;
    const review = document.getElementById('review').value;

    const response = await fetch('/submit_review', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, review })
    });

    const result = await response.text();
    document.getElementById('responseMessage').innerText = result;

    // Reset the form after submission
    document.getElementById('reviewForm').reset();
    form.style.display = 'none';
    overview.style.display ="block";

});


function getColor(vote_average){
    if (vote_average >=8){
        return "green";
    }else if (vote_average >=5) {
        return "orange";
    } else {
        return "red";
    }
}