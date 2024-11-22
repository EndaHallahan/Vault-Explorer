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
