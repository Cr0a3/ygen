// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="intro.html">Getting Started</a></li><li class="chapter-item expanded affix "><li class="part-title">Contributing to ygen</li><li class="chapter-item expanded "><a href="cont/issues.html"><strong aria-hidden="true">1.</strong> Issues</a></li><li class="chapter-item expanded "><a href="cont/prs.html"><strong aria-hidden="true">2.</strong> Pull Requests</a></li><li class="chapter-item expanded "><a href="cont/new_features.html"><strong aria-hidden="true">3.</strong> Adding new ir nodes</a></li><li class="chapter-item expanded affix "><li class="part-title">Architecture of ygen</li><li class="chapter-item expanded "><a href="arch/frontend.html"><strong aria-hidden="true">4.</strong> Frontend</a></li><li class="chapter-item expanded "><a href="arch/opt_pass.html"><strong aria-hidden="true">5.</strong> Optimization</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="arch/opt_pass.html"><strong aria-hidden="true">5.1.</strong> Pass</a></li><li class="chapter-item expanded "><a href="arch/opt_npass.html"><strong aria-hidden="true">5.2.</strong> Implementing a new one</a></li><li class="chapter-item expanded "><a href="arch/passes.html"><strong aria-hidden="true">5.3.</strong> Usable passes</a></li></ol></li><li class="chapter-item expanded "><a href="arch/lower.html"><strong aria-hidden="true">6.</strong> Lowering</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="arch/machine_instr.html"><strong aria-hidden="true">6.1.</strong> MachineInstr</a></li><li class="chapter-item expanded "><a href="arch/mc_instr.html"><strong aria-hidden="true">6.2.</strong> MCInstr</a></li><li class="chapter-item expanded "><a href="arch/backends.html"><strong aria-hidden="true">6.3.</strong> Backends</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="arch/target_descr.html"><strong aria-hidden="true">6.3.1.</strong> TargetDescr</a></li><li class="chapter-item expanded "><a href="arch/backend_structure.html"><strong aria-hidden="true">6.3.2.</strong> Structure</a></li><li class="chapter-item expanded "><a href="tutorials/implemeneting_an_new_backend.html"><strong aria-hidden="true">6.3.3.</strong> Implementing an new backend</a></li></ol></li></ol></li><li class="chapter-item expanded "><li class="part-title">The code of ygen</li><li class="chapter-item expanded "><a href="tutorials/implementing_an_new_ir_node.html"><strong aria-hidden="true">7.</strong> Implemeneting an new ir node</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);