// static/js/blog.js

document.addEventListener("DOMContentLoaded", () => {
  // --- CONFIGURATION ---
  const POSTS_PER_PAGE = 6; // Set how many posts you want per page

  // --- ELEMENT SELECTORS ---
  const categoryFilter = document.getElementById("category-filter");
  const sortFilter = document.getElementById("sort-filter");
  const searchInput = document.getElementById("blog-search");
  const blogGrid = document.querySelector(".blog-grid");
  const paginationContainer = document.getElementById("pagination-container");

  // Exit if we're not on the blog index page
  if (!blogGrid) return;

  const allCards = Array.from(blogGrid.querySelectorAll(".blog-card"));
  let currentPage = 1;
  let currentFilteredCards = [...allCards];

  // --- CORE PAGINATION LOGIC ---

  function displayPage(page) {
    currentPage = page;
    const startIndex = (page - 1) * POSTS_PER_PAGE;
    const endIndex = startIndex + POSTS_PER_PAGE;

    // Hide all cards first
    allCards.forEach(card => card.style.display = 'none');
    
    // Show only the cards for the current page
    const pageCards = currentFilteredCards.slice(startIndex, endIndex);
    pageCards.forEach(card => card.style.display = 'flex'); // Use 'flex' or 'block'

    updatePaginationUI();
  }

  function setupPagination() {
    paginationContainer.innerHTML = ""; // Clear old pagination
    const pageCount = Math.ceil(currentFilteredCards.length / POSTS_PER_PAGE);
    
    if (pageCount <= 1) return; // Don't show pagination if there's only one page

    // Previous Button
    const prevButton = document.createElement('button');
    prevButton.textContent = '← Previous';
    prevButton.className = 'pagination-btn';
    prevButton.disabled = currentPage === 1;
    prevButton.addEventListener('click', () => {
      if (currentPage > 1) displayPage(currentPage - 1);
    });
    paginationContainer.appendChild(prevButton);

    // Page Number Buttons
    for (let i = 1; i <= pageCount; i++) {
      const pageButton = document.createElement('button');
      pageButton.textContent = i;
      pageButton.className = 'pagination-number';
      if (i === currentPage) {
        pageButton.classList.add('active');
      }
      pageButton.addEventListener('click', () => displayPage(i));
      paginationContainer.appendChild(pageButton);
    }

    // Next Button
    const nextButton = document.createElement('button');
    nextButton.textContent = 'Next →';
    nextButton.className = 'pagination-btn';
    nextButton.disabled = currentPage === pageCount;
    nextButton.addEventListener('click', () => {
      if (currentPage < pageCount) displayPage(currentPage + 1);
    });
    paginationContainer.appendChild(nextButton);
  }
  
  function updatePaginationUI() {
      // This is called by displayPage to update button states
      setupPagination();
  }


  // --- FILTERING AND SORTING LOGIC ---

  function applyFiltersAndSort() {
    const category = categoryFilter.value;
    const sortBy = sortFilter.value;
    const query = searchInput.value.toLowerCase();

    // 1. Filter cards
    currentFilteredCards = allCards.filter(card => {
      const title = (card.querySelector("h2 a")?.textContent || "").toLowerCase();
      const cardCategory = (card.dataset.category || "all").toLowerCase();
      return title.includes(query) && (category === "all" || cardCategory === category);
    });

    // 2. Sort the filtered cards
    currentFilteredCards.sort((a, b) => {
      const dateA = new Date(a.dataset.date);
      const dateB = new Date(b.dataset.date);
      return sortBy === "newest" ? dateB - dateA : dateA - dateB;
    });

    // 3. Reset to page 1 and display the results
    displayPage(1);
  }

  // --- INITIALIZATION ---

  // Add event listeners
  if (categoryFilter) categoryFilter.addEventListener("change", applyFiltersAndSort);
  if (sortFilter) sortFilter.addEventListener("change", applyFiltersAndSort);
  if (searchInput) searchInput.addEventListener("input", applyFiltersAndSort);

  // Initial setup on page load
  applyFiltersAndSort();
});