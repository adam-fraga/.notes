interface IObservable {
  //Add an observer
  add(observer: IObserver);
  //Remove an observer
  remove(observer: IObserver);
  //Notify all the Observer calling update() from them
  notify();
}

interface IObserver {
  //Need to know the state (can be many)
  //Update the observer
  update();
}

//Weather station give temperature to subscribers (screen)
class WeatherStation implements IObservable {
  obs: [];
  temperature: Number;

  add() {}
  remove() {}
  notify() {}

  getTemperature() {}
  setTemperature() {}
}

//Multiple Screen  display temperature from WeatherStation
class PhoneDisplay implements IObserver {
  //Reference to the Observable
  constructor(station: WeatherStation) {}
  update() {}
}

//Multiple Screen  display temperature from WeatherStation
class WindowDisplay implements IObserver {
  //Reference to the Observable
  constructor(station: WeatherStation) {}
  update() {}
}
