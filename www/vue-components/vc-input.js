Vue.component('vc-input',
  {
    data: function() {
      return {};
    },
    props: ['value', 
            'type',
            'placeholder',
             'disabled'],
    computed: {
      input: {
        get: function() { return this.value; },
        set: function(nV) { this.$emit('input', nV); }
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
