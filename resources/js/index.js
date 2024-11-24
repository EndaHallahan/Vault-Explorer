import '../sass/index.scss';

const renderComplete = new Event('rendercomplete');

window.addEventListener("load", () => {
	dataBinder();
	dispatchEvent(renderComplete);
}, false);

// Attatches controllers to page elements
function dataBinder() {
	console.log("Binding elements!");

	let unboundControllers = document.querySelectorAll("[data-unbound]");
	unboundControllers.forEach(ele => {
		switch (ele.dataset.unbound) {
			case "nav-aside":
				new NavAside(ele);
				break;
			case "expand-parent":
				new ExpandParent(ele);
				break;
			case "search":
				new Search(ele);
				break;
			default:
		}
		ele.removeAttribute("data-unbound");
	});
}


class NavAside {
	constructor(element) {
		this.boundEle = element;
		this.collapseButton = this.boundEle.querySelector("button.aside-collapser");
		this.collapseButton.addEventListener("click", this.collapseUncollapse.bind(this), false);
	}

	collapseUncollapse() {
		this.boundEle.classList.toggle("aside-collapsed");
	}
}

class ExpandParent {
	constructor(element) {
		this.boundEle = element;
		this.expandableChild = this.boundEle.querySelector(".expandable");
		this.expandButton = this.boundEle.querySelector(":scope > button.child-expander");
		if (this.expandableChild !== null && this.expandButton !== null) {
			this.expandButton.addEventListener("click", this.collapseUncollapse.bind(this), false);
		}
		if (this.boundEle.querySelector(".force-expanded") !== null) {
			this.expandableChild.classList.add("expanded");
		}
	}

	collapseUncollapse() {
		this.expandableChild.classList.toggle("expanded");
	}
}

class Search {
	constructor(element) {
		this.boundEle = element;
		this.searchBar = element.querySelector("input[type=text]");
		this.submitButton = element.querySelector("button[type=submit]");
		this.errOut = element.querySelector(".err-out");
		this.resultsOut = element.querySelector(".search-results-out");
		this.resultTemplate = element.querySelector("template.search-result-template");
		this.disableable = this.boundEle.querySelectorAll(".disableable");
		this.vault = element.dataset.vault;

		element.addEventListener("submit", (event) => {
			event.preventDefault();
			this.postForm();
		});
	}

	async postForm() {
		this.disableable.forEach(ele => {
			ele.disabled = true;
		});
		let payload = "query=" + encodeURIComponent(this.searchBar.value);
		let data = await getAjax("/api/search", payload);
		this.disableable.forEach(ele => {
			ele.disabled = false;
		});
		try {
			if (data.success) {
				this.resultsOut.textContent = "";
				const resultsFrag = new DocumentFragment();
				console.log(data.results);
				data.results.forEach(result => {
					let resultClone = this.resultTemplate.content.cloneNode(true);

					resultClone.querySelector(".title-field").innerText = result.title;
					resultClone.querySelector(".body-field").innerHTML = result.body;
					
					resultsFrag.append(resultClone);
				});
				this.resultsOut.append(resultsFrag);
			} else {
				console.log(data);
				if (this.errOut) {
					if (data.msg) {
						this.errOut.innerText = data.msg;
					} else {
						this.errOut.innerText = "Something went wrong on our end. Try again later.";
					}
				}
			}
		} catch (e) {
	    	console.error(e);
	    }
	}
}

async function getAjax(url, payload) {
	 try {
    	const response = await fetch(url + "?" + payload, {
			method: "GET",
			credentials: "same-origin",
			headers: {
	            "Accept": "application/json",
	            "Content-Type": "application/json",
	        }
	    });
	    if (!response.ok) {
	    	console.error("Network response was not OK");
	    }
	    const data = await response.json();
	    return data;
    } catch (e) {
    	return e;
    }
}

async function postAjax(url, payload) {
	 try {
    	const response = await fetch(url, {
			method: "POST",
			credentials: "same-origin",
			headers: {
	            "Accept": "application/json",
	            "Content-Type": "application/json",
	        },
	        body: JSON.stringify(payload)
	    });
	    if (!response.ok) {
	    	console.error("Network response was not OK");
	    }
	    const data = await response.json();
	    return data;
    } catch (e) {
    	return e;
    }
}
