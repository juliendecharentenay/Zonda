Vue.component('vc-input',
  {
    data: function() {
      return {};
    },
    props: ['value', 
            'type',
            'parse',
            'placeholder',
             'disabled'],
    computed: {
      input: {
        get: function() { return this.value; },
        set: function(nV) { 
          switch (this.parse) {
            case "float":
              nV = parseFloat(nV);
              break;
            case "int":
              nV = parseInt(nV);
              break;
          }
          this.$emit('change', {from: this.value, to: nV});
          this.$emit('input', nV); 
        }
      }
    },
    template: `
<div class="field">
  <div class="label is-small"><slot></slot></div>
    <div class="control">
      <input class="input is-small" 
             v-bind:type="type" 
             v-bind:placeholder="placeholder"
             v-model="input"
             v-bind:disabled="disabled">
    </div>
  </div>
</div>

`
});
