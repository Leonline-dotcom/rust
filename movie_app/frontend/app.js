


const main = document.getElementById('main');
const searchform =  document.getElementById('searchForm');
const search = document.getElementById('search');
const form = document.querySelector('.movie form');
const overview = document.querySelector('.overview');

const BASE_IMG_URL ="https://image.tmdb.org/t/p/w500"; 


get_movies();
async function get_movies(){
    try {
        const response = await fetch('/movies');
        const result = await response.json();
        console.log(result);  
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
                <input type="text" placeholder="Enter Name" class="usernameInput" required>
                <textarea class="reviewInput" placeholder="How was the movie?" required></textarea>
                <button type="submit" class="submit_review_btn">Submit Review</button>
            </form>
        `;

        // log a click!
        movieEl.addEventListener('click', () => {
            showForm(movieEl, title);  // Pass the title to showForm
        });

         // Add submit event listener for each movie's review form
         const submitBtn = movieEl.querySelector('.submit_review_btn');
         submitBtn.addEventListener('click', async function (event) {
             event.preventDefault();
             const usernameInput = movieEl.querySelector('.usernameInput');
             const reviewInput = movieEl.querySelector('.reviewInput');
             const username = usernameInput.value;
             const review = reviewInput.value;
             if ((username != "")&&(review !="")){
             const posterpath = BASE_IMG_URL + poster_path;
             const response = await fetch('/submit_review', {
                 method: 'POST',
                 headers: { 'Content-Type': 'application/json' },
                 body: JSON.stringify({ title, username, review,posterpath })
             });
 
             const result = await response.text();
             console.log(result);
             }
             // Reset the form after submission
             movieEl.querySelector('form').reset();
             movieEl.querySelector('form').style.display = 'none';
             movieEl.querySelector('.overview').style.display = 'block';
         });
        main.appendChild(movieEl);
    });
}

function showForm(movieEl, title) {
    const form = movieEl.querySelector('form');
    const overview = movieEl.querySelector('.overview');
    console.log("Clicked on movie: " + title); // Log the movie title
    form.style.display = 'block';
    overview.style.display ='none';
}   

function getColor(vote_average){
    if (vote_average >=8){
        return "green";
    }else if (vote_average >=5) {
        return "orange";
    } else {
        return "red";
    }
}

document.querySelectorAll('.horizontalContainer button').forEach(button => {
    button.addEventListener('click', async () => {
        console.log("clicked");
        button.classList.toggle('clicked');
        if(button.classList.contains('clicked')){
            try {
                const response = await fetch("/reviews");
                const reviews = await response.json();
                console.log(reviews);
                showReviews(reviews); 
            } catch (error) {
                console.log("Couldn't reach mongo");
            }
        } else {
            get_movies();
        }
    });
});


async function showReviews(reviews) {
    console.log("making review panels");
    main.innerHTML = '';
    const reviewContainer = document.createElement('div');
    reviewContainer.className ='reviewPanelContainer';
    reviews.reverse().forEach(reviewinfo => {
        const { title, username, review , posterpath} =reviewinfo;
        const review_panelEL = document.createElement('div');
        review_panelEL.className = "reviewPanel"
        review_panelEL.innerHTML =`
        <div class ="reviewLayout">
            <div class="title">${title}</div>
            <br>
            <div class="review">${review}</div>
            <div class="username"> -${username}</div>
        </div>
        <img class="poster" src="${posterpath}" alt="${title}"></img>`;
      reviewContainer.appendChild(review_panelEL);  
    });
    main.appendChild(reviewContainer);
}


searchform.addEventListener('submit', (e) =>{
    e.preventDefault();

    const searchTerm= search.value;
    if (searchTerm){
        search_movie(searchTerm);
    }else{
        get_movies();
    }
});

async function search_movie(searchTerm) {

    const searchStr = "/movies/search/"+searchTerm;
    try {
        const response = await fetch(searchStr, {
            method: 'GET',
            headers: { 'Content-Type': 'application/json' },
        });

        const result = await response.json();
        console.log(result);  // Output the fetched data
        showMovies(result.results);

    } catch (error) {
        console.error('Error:', error);
    }
}


