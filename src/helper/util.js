export class Util {

  static groupBy(arrOfObjects, objKey) {
    return arrOfObjects.reduce((map, obj) => {
      const k = obj[objKey];

      // init map at key as array
      if (!map[k]) {
        map[k] = [];
      }

      // add our obj to map
      map[k].push(obj);
      return map;
    }, {});
  }

}
