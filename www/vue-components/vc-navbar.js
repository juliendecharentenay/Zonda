Vue.component('vc-navbar',
  {
    data: function() {
      return {};
    },
    template: `
<nav class="navbar is-fixed-top is-light" role="navigation">
  <div class="navbar-brand">
    <div class="navbar-item">Project Zonda</div>
  </div>

  <div class="navbar-end">
    <a href="/simulation.html" class="navbar-item">New Simulation</a>
  </div>
</nav>
`
});
