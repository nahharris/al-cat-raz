fn on_hit(attacker, victim) {
    let d = attacker.get_pos() - victim.get_pos();
    victim.move(d);
}