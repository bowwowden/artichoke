# frozen_string_literal: true

# Exception raised by a throw
class UncaughtThrowError < ArgumentError
  # @!attribute [r] tag
  #   @return [Symbol] tag object, mostly a Symbol
  attr_reader :tag
  # @!attribute [r] value
  #   @return [Array] extra parameters passed in
  attr_reader :value

  # @param [Symbol] tag  object to throw
  # @param [Object] value  any object to return to the catch block
  def initialize(tag, value = nil)
    @tag = tag
    @value = value
    super "uncaught throw #{tag}"
  end
end

module Kernel
  # Setup a catch block and wait for an object to be thrown, the
  # catch end without catching anything.
  #
  # @param [Symbol] tag  tag to catch
  # @return [void]
  #
  # @example
  #   catch :thing do
  #     __do_stuff__
  #     throw :thing
  #   end
  def catch(tag = nil)
    tag = Object.new if tag.nil?

    yield tag
  rescue UncaughtThrowError => e
    raise e unless e.tag == tag

    e.value
  end

  def Array(arg) # rubocop:disable Naming/MethodName
    return arg if arg.is_a?(Array)

    ret = nil
    ret = arg.to_ary if arg.respond_to?(:to_ary)
    ret = arg.to_a if ret.nil? && arg.respond_to?(:to_a)

    return [arg] if ret.nil?
    return ret if ret.is_a?(Array)

    raise TypeError
  end

  def Hash(arg) # rubocop:disable Naming/MethodName
    return arg if arg.is_a?(Hash)
    return {} if arg.nil? || arg == []

    if arg.respond_to?(:to_hash)
      ret = arg.to_hash
      return {} if ret.nil? || ret.empty?
      return ret if ret.is_a?(Hash)
    end

    raise TypeError
  end

  # TODO: Not fully implemented. Just enough for Array spec to pass.
  def Integer(arg, base = 0, exception: true) # rubocop:disable Naming/MethodName
    raise ArgumentError, "invalid radix #{base}" unless base.zero? || (2..36).include?(base)
    raise ArgumentError, 'base specified for non string value' if base.positive? && arg.is_a?(String)

    return arg.to_int if arg.respond_to?(:to_int)
    return arg.to_i if arg.respond_to?(:to_i)

    0
  end

  def String(arg) # rubocop:disable Naming/MethodName
    return arg if arg.is_a?(String)
    # TODO: after defined? keyword is implemented
    # if `to_s` is not defined, raise TypeError even if respond_to?(:to_s) return true
    if arg.respond_to?(:to_s)
      ret = arg.to_s
      return ret if ret.is_a?(String)
    end

    raise TypeError
  end

  # Throws an object, uncaught throws will bubble up through other catch blocks.
  #
  # @param [Symbol] tag  tag being thrown
  # @param [Object] value  a value to return to the catch block
  # @raises [UncaughtThrowError]
  # @return [void] it will never return normally.
  #
  # @example
  #   catch :ball do
  #     pitcher.wind_up
  #     throw :ball
  #   end
  def throw(tag, value = nil)
    raise UncaughtThrowError.new(tag, value)
  end
end
