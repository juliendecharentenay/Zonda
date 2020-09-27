/** libutils.js
 *  Set of utilities that are taken from various web resources 
 */

var libUtils = {

  /** Generate uuid
   * From: https://stackoverflow.com/questions/105034/how-to-create-guid-uuid
   */
  uuidv4: function() {
      return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
        var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
  }

};
