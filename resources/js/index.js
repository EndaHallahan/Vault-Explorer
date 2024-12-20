import '../sass/index.scss';
import 'highlight.js/styles/github-dark.css';
import hljs from "highlight.js";

const renderComplete = new Event('rendercomplete');

window.addEventListener("load", () => {
	dataBinder();
	hljs.highlightAll();
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
			case "tag":
				new Tag(ele);
				break;
			case "side-nav":
				new SideNav(ele);
				break;
			default:
		}
		ele.removeAttribute("data-unbound");
	});
}


class NavAside {
	constructor(element) {
		this.boundEle = element;
		this.closed = this.boundEle.classList.contains("aside-collapsed");
		this.collapseButton = this.boundEle.querySelector("button.aside-collapser");
		this.collapseButton.addEventListener("click", this.collapseUncollapse.bind(this), false);
		window.addEventListener("openAsideNav", (event) => {
			if (this.closed) {
				this.collapseUncollapse();
			}
		});
	}

	collapseUncollapse() {
		this.boundEle.classList.toggle("aside-collapsed");
		this.closed = !this.closed;
		window.localStorage.setItem("side-nav-closed", this.closed)
		//setCookie("side-nav-closed", this.closed, 365);
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
			window.history.pushState(null, null, "?query=" + encodeURIComponent(this.searchBar.value));
			this.getResults();
		});

		window.addEventListener('popstate', this.pullFromURL.bind(this));

		window.addEventListener("searchFor", (event) => {
			this.searchBar.value = event.detail;
			window.history.pushState(null, null, "?query=" + encodeURIComponent(this.searchBar.value));
			this.getResults();
		});

		if (this.searchBar.value) {
			this.getResults();
		}
	}

	pullFromURL() {
		const urlSearchParams = new URLSearchParams(window.location.search);
		if (urlSearchParams.has("query")) {
			this.searchBar.value = urlSearchParams.get("query");
			this.getResults();
		}
	}

	async getResults() {
		this.disableable.forEach(ele => {
			ele.disabled = true;
		});
		let val = this.searchBar.value;
		if (this.vault) {
			val = "vault:\"" + this.vault +"\" AND " + val;
		}
		let payload = "query=" + encodeURIComponent(val);
		let data = await getAjax("/api/search", payload);
		this.disableable.forEach(ele => {
			ele.disabled = false;
		});
		try {
			if (data.success) {
				this.resultsOut.textContent = "";
				this.errOut.innerText = "";
				const resultsFrag = new DocumentFragment();
				//console.log(data.results);
				if (data.results.length === 0) {
					resultsFrag.append(eleBuilder("H2", {text: "No results found."}));
				}
				data.results.forEach(result => {
					let resultClone = this.resultTemplate.content.cloneNode(true);
					let titleField = resultClone.querySelector(".title-field");
					let vaultField = resultClone.querySelector(".vault-field");
					let bodyField = resultClone.querySelector(".body-field");
					let tagsField = resultClone.querySelector(".tags-field");

					if (titleField !== null) {
						let titleLink = eleBuilder("A", {
							href: "/vault/" + pathify(result.vault) + "/note/" + pathify(result.title),
							text: result.title
						});
						titleField.append(titleLink);
					}
					
					if (vaultField !== null) {
						let vaultLink = eleBuilder("A", {
							href: "/vault/" + pathify(result.vault),
							text: result.vault
						});
						vaultField.append(vaultLink);
					}
					
					if (bodyField !== null) {
						bodyField.innerHTML = result.body;
					}
					

					if (tagsField !== null) {
						result.tags.forEach(tag => {
							let tagEle = eleBuilder("LI", {
								text: "#" + tag, 
								class: "tag",
								data: ["tag-name", tag]
							});
							new Tag(tagEle);
							tagsField.append(tagEle);
						});
					}
						
					resultsFrag.append(resultClone);
				});
				this.resultsOut.append(resultsFrag);
			} else {
				console.log(data);
				if (this.errOut) {
					if (data.message) {
						this.errOut.innerText = data.message;
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

class Tag {
	constructor(element) {
		this.boundEle = element;
		this.tagName = this.boundEle.dataset.tagName;
		this.boundEle.addEventListener("click", this.searchMe.bind(this), false);
	}
	searchMe() {
		const searchFor = new CustomEvent("searchFor", { detail: "tag:\"" + this.tagName + "\"" });
		window.dispatchEvent(searchFor);
		const openAsideNav = new Event('openAsideNav');
		window.dispatchEvent(openAsideNav);
	}	
}

class SideNav {
	constructor(element) {
		this.boundEle = element;
		this.boundEle.querySelectorAll(".tab-headers > button").forEach(button => {
			button.addEventListener("click", e => {
				this.switchTab(e.target.dataset.tab);
			}, false);
		});
		window.addEventListener("searchFor", (event) => {
			this.switchTab("search");
		});
	}

	switchTab(tabName) {
		this.boundEle.dataset.tab = tabName;
	}
}





function setCookie(cname, cvalue, exdays) {
	const d = new Date();
	d.setTime(d.getTime() + (exdays*24*60*60*1000));
	let expires = "expires="+ d.toUTCString();
	document.cookie = cname + "=" + cvalue + ";" + expires + ";path=/";
}

function getCookie(cname) {
	let name = cname + "=";
	let decodedCookie = decodeURIComponent(document.cookie);
	let ca = decodedCookie.split(';');
	for(let i = 0; i <ca.length; i++) {
		let c = ca[i];
		while (c.charAt(0) == ' ') {
			c = c.substring(1);
		}
		if (c.indexOf(name) == 0) {
			return c.substring(name.length, c.length);
		}
	}
	return "";
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
function eleBuilder(eleStr, propObj) {
    const ele = document.createElement(eleStr);
    if (typeof propObj !== "undefined") {
    	if (propObj.class) {ele.className = propObj.class;}
	    if (propObj.data) {ele.setAttribute("data-" + propObj.data[0], propObj.data[1])}
	    if (propObj.HTML) {ele.innerHTML = propObj.HTML;}
	    if (propObj.text) {ele.innerText = propObj.text;}
	    if (propObj.id) {ele.id = propObj.id;}
	    if (propObj.type) {ele.type = propObj.type;}
	    if (propObj.value) {ele.value = propObj.value;}
	    if (propObj.style) {ele.setAttribute('style', propObj.style)}
	    if (propObj.href) {ele.setAttribute('href', propObj.href)}
	    if (propObj.event) {ele.addEventListener(propObj.event[0], propObj.event[1], false);}
		if (propObj.checked) {ele.checked = propObj.checked}
	    if (propObj.attributes) {
	    	propObj.attributes.forEach(attribute => {
	    		// [attribute {name, value}]
	    		let att = document.createAttribute(attribute.name);
	    		att.value = attribute.value;
	    		ele.setAttributeNode(att);
	    	});
	    }
    }
    return ele;
}

function pathify(in_string) {
	return in_string.replaceAll(" ", "_").replaceAll('/', "%2F");
}
